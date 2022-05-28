use crate::eval::Expr::{Binary, Grouping, Literal, Unary, Variable};
use crate::parse::Operator;
use crate::parse::{Expr, Stmt, Value};
use std::collections::HashMap;

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
pub struct EvalState<'a> {
    parent: Option<&'a EvalState<'a>>,
    symbols: HashMap<&'a str, Value>,
}

impl<'a> EvalState<'a> {
    pub fn new() -> Self {
        EvalState {
            parent: None,
            symbols: HashMap::new(),
        }
    }
    pub fn new_from_parent(parent: &'a EvalState<'a>) -> Self {
        EvalState {
            parent: Some(parent),
            symbols: HashMap::new(),
        }
    }
    pub fn lookup(self: &Self, key: &str) -> Option<Value> {
        match (self.symbols.get(&key), self.parent) {
            (Some(value), _) => Some(value.clone()),
            (None, Some(parent)) => parent.lookup(key),
            (None, None) => None,
        }
    }
}

pub fn eval_statements(
    stmts: &[Stmt],
    parent_eval_state: &mut EvalState,
) -> Result<(), RuntimeError> {
    let mut eval_state = EvalState::new_from_parent(parent_eval_state);

    for stmt in stmts {
        match stmt {
            Stmt::VarDecl(id, expr) => match eval_expression(expr, &eval_state) {
                Ok(value) => {
                    eval_state.symbols.insert(id, value);
                }
                Err(err) => return Err(err),
            },
            Stmt::Block(stmts) => {
                eval_statements(stmts, &mut eval_state)?;
            }
            Stmt::Print(expr) => match eval_expression(expr, &mut eval_state) {
                Ok(value) => println!("{}", value),
                Err(err) => return Err(err),
            },
            Stmt::Expression(expr) => match eval_expression(expr, &mut eval_state) {
                Ok(_) => (),
                Err(err) => return Err(err),
            },
        }
    }
    Ok(())
}

#[rustfmt::skip]
pub fn eval_expression(expr: &Expr, eval_state: &EvalState) -> EvalResult {
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
            let left = eval_expression(left,eval_state)?;
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
          match eval_state.lookup(id) {
              Some(value) => Ok(value.clone()),
              None => Err(RuntimeError{message:format!("Using unknown symbol {}", id)}),
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
