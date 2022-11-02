use crate::eval::Expr::{Assign, Binary, Call, Grouping, Literal, Logical, Unary, Variable};
use crate::parse::Operator;
use crate::parse::{Expr, Stmt, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct RuntimeError(String);

// All values have a true or false value. The only things that are false in lox are nil and
// boolean false, everything else is true
// TODO it's really an error if this is not a value so maybe this should return RuntimeError?
fn bool_value(value: &Value) -> bool {
    !(matches!(value, Value::Boolean(false)) || matches!(value, Value::Nil))
}

fn numeric_value(value: &Value) -> Option<f64> {
    match value {
        Value::Number(num) => Some(*num),
        _ => None,
    }
}

type EvalResult = Result<Value, RuntimeError>;

#[derive(Debug)]
pub struct EvalState {
    parent: Option<Rc<RefCell<EvalState>>>,
    symbols: HashMap<String, Option<Value>>,
}

impl Default for EvalState {
    fn default() -> Self {
        Self::new()
    }
}

impl EvalState {
    pub fn new() -> Self {
        EvalState {
            parent: None,
            symbols: HashMap::new(),
        }
    }
    pub fn new_from_parent(parent: Rc<RefCell<EvalState>>) -> Self {
        EvalState {
            parent: Some(parent),
            symbols: HashMap::new(),
        }
    }
    /// lookup finds the key in the current block's symbol table and
    /// then looks in the parent table and so on until it runs out of
    /// places to look
    pub fn lookup(&self, key: &str) -> EvalResult {
        match (self.symbols.get(&key.to_string()), &self.parent) {
            (Some(Some(value)), _) => Ok(value.clone()),
            (Some(None), _) => Err(RuntimeError(format!(
                "Unitialized variable access: {}",
                key
            ))),
            (None, Some(parent)) => parent.borrow().lookup(key),
            (None, None) => Err(RuntimeError(format!("Unknown variable access: {}", key))),
        }
    }
    /// assign gives variable `key` the value `value`, finding the variable
    /// in the same way that lookup does
    pub fn assign(&mut self, key: &str, value: &Value) -> EvalResult {
        let key_string = key.to_string();
        let found = self.symbols.get(&key_string).is_some();

        if found {
            self.symbols.insert(key_string, Some(value.clone()));
            Ok(value.clone())
        } else {
            match &self.parent {
                Some(p) => p.borrow_mut().assign(key, value),
                None => Err(RuntimeError(format!(
                    "Assignent to unknown variable {}",
                    key
                ))),
            }
        }
    }
}

pub fn eval_statements(
    stmts: &[Stmt],
    parent_eval_state: Rc<RefCell<EvalState>>,
) -> Result<(), RuntimeError> {
    let eval_state = Rc::new(RefCell::new(EvalState::new_from_parent(Rc::clone(
        &parent_eval_state,
    ))));

    for stmt in stmts {
        match stmt {
            Stmt::VarDecl(id, Some(expr)) => match eval_expression(expr, Rc::clone(&eval_state)) {
                Ok(value) => {
                    eval_state
                        .borrow_mut()
                        .symbols
                        .insert(id.to_string(), Some(value));
                }
                Err(err) => return Err(err),
            },
            Stmt::VarDecl(id, None) => {
                eval_state.borrow_mut().symbols.insert(id.to_string(), None);
            }
            Stmt::Block(stmts) => {
                eval_statements(stmts, Rc::clone(&eval_state))?;
            }
            Stmt::Print(expr) => match eval_expression(expr, Rc::clone(&eval_state)) {
                Ok(value) => println!("{}", value),
                Err(err) => return Err(err),
            },
            Stmt::Expression(expr) => match eval_expression(expr, Rc::clone(&eval_state)) {
                Ok(_) => (),
                Err(err) => return Err(err),
            },
            Stmt::If(expr, then_stmt, else_stmt) => {
                let cond = eval_expression(expr, Rc::clone(&eval_state))?;
                let cond_bool = bool_value(&cond);
                if cond_bool {
                    eval_statements(then_stmt, Rc::clone(&eval_state))?
                } else {
                    eval_statements(else_stmt, Rc::clone(&eval_state))?
                }
            }
            Stmt::While(expr, stmts) => loop {
                let cond = eval_expression(expr, Rc::clone(&eval_state))?;
                let cond_bool = bool_value(&cond);
                if cond_bool {
                    eval_statements(stmts, Rc::clone(&eval_state))?
                } else {
                    break;
                }
            },
        }
    }
    Ok(())
}

#[rustfmt::skip]
pub fn eval_expression(expr: &Expr, eval_state: Rc<RefCell<EvalState>>) -> EvalResult {
    match expr {
        Literal(value) => Ok(value.clone()),
        Call(callee, arguments) => todo!(),
        Unary(operator, right) => {
            let right = eval_expression(right,eval_state)?;
            match operator {
                Operator::Bang => {
                    let b = bool_value(&right);
                    Ok(Value::Boolean(!b))
                },
                Operator::Minus => {
                    match numeric_value(&right) {
                        Some(n) =>
                            Ok(Value::Number(-n)),
                        None => 
                            Err(RuntimeError(format!("Cannot negate {:?}", right)))
                    }
                },
                thing  => {
                    Err(RuntimeError(format!("Unary inappropriate for {:?}", thing)))
                },
            }
        },
        Binary(left, operator, right) => {
            let left = eval_expression(left,Rc::clone(&eval_state))?;
            let right = eval_expression(right,eval_state)?;
            let left_number = numeric_value(&left);
            let right_number = numeric_value(&right);

            match operator {
                // String concat
                Operator::Plus if matches!(left,Value::String(_)) => eval_string_append(left,right),
                // Equality operators
                Operator::EqualEqual => eval_equality_operator(left,right,false),
                Operator::BangEqual => eval_equality_operator(left,right,true),
                // Comparison operators
                Operator::Greater => eval_comparison_operator(left,right,left_number,right_number,">",|(a,b)| a > b),
                Operator::GreaterEqual => eval_comparison_operator(left,right,left_number,right_number,">=",|(a,b)| a >= b),
                Operator::Less => eval_comparison_operator(left,right,left_number,right_number,"<",|(a,b)| a < b),
                Operator::LessEqual => eval_comparison_operator(left,right,left_number,right_number,"<=",|(a,b)| a <= b),
                // Arithmetic
                Operator::Minus => eval_arithmetic_operator( left, right, left_number, right_number, "-", |(a, b)| a - b),
                Operator::Plus => eval_arithmetic_operator( left, right, left_number, right_number, "+", |(a, b)| a + b),
                Operator::Star => eval_arithmetic_operator( left, right, left_number, right_number, "*", |(a, b)| a * b),
                Operator::Slash => eval_arithmetic_operator( left, right, left_number, right_number, "/", |(a, b)| a / b),
                _ => todo!(),
            }
        },
        Logical(left,operator,right) => {
            let left = eval_expression(left,Rc::clone(&eval_state))?;
            match operator {
                Operator::And if !bool_value(&left) => Ok(left),
                Operator::Or if bool_value(&left) => Ok(left),
                Operator::Or | Operator::And => {
                    eval_expression(right,Rc::clone(&eval_state))
                },
                _ => Err(RuntimeError(format!("Unexpected logical operator : {}", operator)))
            }
        },
        Grouping(expr) => eval_expression(expr,Rc::clone(&eval_state)),
        Variable(id) => {
          match eval_state.borrow().lookup(id) {
              Ok(value) => Ok(value),
              err @ Err(_) => err,
          }
        },
        Assign(id, expr) => {
            let value = eval_expression(expr, Rc::clone(&eval_state))?;
            eval_state.borrow_mut().assign(id,&value)?;
            Ok(value)
        },
    }
}

// Nil is only equal to nil
// Two numbers can be compared
// Two bools can be compared
// Otherwise it is not equal
fn eval_equality_operator(left: Value, right: Value, negate: bool) -> EvalResult {
    let result = match (&left, &right) {
        (Value::Nil, Value::Nil) => true,
        (Value::Number(n1), Value::Number(n2)) => n1 == n2,
        (Value::Boolean(b1), Value::Boolean(b2)) => b1 == b2,
        (Value::String(s1), Value::String(s2)) => s1 == s2,
        _ => {
            return Err(RuntimeError(format!(
                "Don't know how to compare {:?} and {:?}",
                left, right
            )))
        }
    };

    if negate {
        Ok(Value::Boolean(!result))
    } else {
        Ok(Value::Boolean(result))
    }
}

fn eval_arithmetic_operator<T>(
    left: Value,
    right: Value,
    left_number: Option<f64>,
    right_number: Option<f64>,
    text: &str,
    f: T,
) -> EvalResult
where
    T: Fn((f64, f64)) -> f64,
{
    match left_number.zip(right_number).map(f) {
        Some(result) => Ok(Value::Number(result)),
        None => Err(RuntimeError(format!(
            "Arithmetic error: {:?} {:?} {:?}",
            left, text, right
        ))),
    }
}

fn eval_comparison_operator<T>(
    left: Value,
    right: Value,
    left_number: Option<f64>,
    right_number: Option<f64>,
    text: &str,
    f: T,
) -> EvalResult
where
    T: Fn((f64, f64)) -> bool,
{
    match left_number.zip(right_number).map(f) {
        Some(result) => Ok(Value::Boolean(result)),
        None => Err(RuntimeError(format!(
            "Comparison error: {:?} {:?} {:?}",
            left, text, right
        ))),
    }
}

fn eval_string_append(left: Value, right: Value) -> EvalResult {
    match (&left, &right) {
        (Value::String(s1), Value::String(s2)) => Ok(Value::String(format!("{}{}", s1, s2))),
        _ => Err(RuntimeError(format!("Cannot string append {:?}", right))),
    }
}
