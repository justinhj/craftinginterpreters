use crate::eval::Expr::{Binary, Grouping, Literal, Unary, Variable};
use crate::parse::Operator;
use crate::parse::{Expr, Stmt, Value};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
}

// All values have a true or false value. The only things that are false in lox are nil and
// boolean false, everything else is true
// TODO it's really an error if this is not a value so maybe this should return RuntimeError?
fn bool_value(value: &Value) -> bool {
    if matches!(value, Value::Boolean(false)) || matches!(value, Value::Nil) {
        false
    } else {
        true
    }
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
    pub fn lookup(self: &Self, key: &str) -> EvalResult {
        match (self.symbols.get(&key.to_string()), &self.parent) {
            (Some(Some(value)), _) => Ok(value.clone()),
            (Some(None), _) => Err(RuntimeError {
                message: format!("Unitialized variable access: {}", key),
            }),
            (None, Some(parent)) => parent.borrow().lookup(key),
            (None, None) => Err(RuntimeError {
                message: format!("Unknown variable access: {}", key),
            }),
        }
    }
}

pub fn eval_statements(
    stmts: &[Stmt],
    parent_eval_state: Rc<RefCell<EvalState>>
) -> Result<(), RuntimeError> {
    let mut eval_state = Rc::new(RefCell::new(EvalState::new_from_parent(Rc::clone(&parent_eval_state))));

    for stmt in stmts {
        match stmt {
            Stmt::VarDecl(id, Some(expr)) => match eval_expression(expr, Rc::clone(&eval_state)) {
                Ok(value) => {
                    eval_state.borrow_mut().symbols.insert(id.to_string(), Some(value));
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
        }
    }
    Ok(())
}

#[rustfmt::skip]
pub fn eval_expression(expr: &Expr, eval_state: Rc<RefCell<EvalState>>) -> EvalResult {
    match expr {
        Literal(value) => Ok(value.clone()),
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
                            Err(RuntimeError{message:format!("Cannot negate {:?}", right)}),
                    }
                },
                thing @ _ => {
                    Err(RuntimeError{message:format!("Unary inappropriate for {:?}", thing)})
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
        Grouping(expr) => eval_expression(expr,eval_state),
        Variable(id) => {
          match eval_state.borrow().lookup(id) {
              Ok(value) => Ok(value.clone()),
              err @ Err(_) => err,
          }
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
            return Err(RuntimeError {
                message: format!("Don't know how to compare {:?} and {:?}", left, right),
            })
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
        None => Err(RuntimeError {
            message: format!("Arithmetic error: {:?} {:?} {:?}", left, text, right),
        }),
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
        None => Err(RuntimeError {
            message: format!("Comparison error: {:?} {:?} {:?}", left, text, right),
        }),
    }
}

fn eval_string_append(left: Value, right: Value) -> EvalResult {
    match (&left, &right) {
        (Value::String(s1), Value::String(s2)) => Ok(Value::String(format!("{}{}", s1, s2))),
        _ => Err(RuntimeError {
            message: format!("Cannot string append {:?}", right),
        }),
    }
}
