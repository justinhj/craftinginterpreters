use crate::eval::Expr::{Binary, Literal};
use crate::parse::Operator;
use crate::parse::{Expr, Value};

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
}

// All values have a true or false value. The only things that are false in lox are nil and
// boolean false, everything else is true
// TODO it's really an error if this is not a value so maybe this should return RuntimeError?
fn bool_value(value: &Value) -> bool {
    if matches!(value, Value::Boolean(false))
        || matches!(value, Value::Nil)
    {
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

#[rustfmt::skip]
pub fn eval(expr: &Expr) -> EvalResult {
    match expr {
        Literal(value) => Ok(value.clone()),
        Binary(left, operator, right) => {
            let left = eval(left)?;
            let right = eval(right)?;
            let left_number = numeric_value(&left);
            let right_number = numeric_value(&right);
            let left_bool = bool_value(&left);
            let right_bool = bool_value(&right);

            match operator {
                // Equality operators
                Operator::EqualEqual => eval_equality_operator(left_bool,right_bool,|(a,b)| a == b),
                Operator::BangEqual => eval_equality_operator(left_bool,right_bool,|(a,b)| a != b),
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
        }
        _ => {
            println!("eval3 {:?}", expr);
            todo!()
        }
    }
}

fn eval_equality_operator<T>( left_bool: bool, right_bool: bool,
    f: T,
) -> EvalResult
where
    T: Fn((bool, bool)) -> bool,
{
    let result = f((left_bool,right_bool));
    Ok(Value::Boolean(result))
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
