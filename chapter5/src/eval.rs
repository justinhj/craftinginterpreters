use crate::eval::Expr::{Binary,Literal};
use crate::parse::{Expr,Value};
use crate::parse::Operator;

#[derive(Debug)]
pub struct RuntimeError {
    message: String,
}

// All values have a true or false value. The only things that are false in lox are nil and
// boolean false, everything else is true
// TODO it's really an error if this is not a value so maybe this should return RuntimeError?
fn is_bool_literal(expr: &Expr) -> bool {
    if matches!(expr, Expr::Literal(Value::Boolean(false))) || matches!(expr, Expr::Literal(Value::Nil)) {
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

type EvalResult = Result<Value,RuntimeError>;

pub fn eval(expr: &Expr) -> EvalResult {
    match expr {
        Literal(value) => Ok(value.clone()),
        Binary(left, operator, right) => {
            let left = eval(left)?;
            let right = eval(right)?;
            let left_number = numeric_value(&left);
            let right_number = numeric_value(&right);

            match operator {
                // Comparison operators
                Operator::Greater => {
                    match left_number.zip(right_number).map(|(a,b)| a > b) {
                        Some(result) => Ok(Value::Boolean(result)),
                        None => Err(RuntimeError{message:format!("Comparison error: > {:?} {:?}", left, right)}),
                    }
                },
                Operator::GreaterEqual => {
                    match left_number.zip(right_number).map(|(a,b)| a >= b) {
                        Some(result) => Ok(Value::Boolean(result)),
                        None => Err(RuntimeError{message:format!("Comparison error: >= {:?} {:?}", left, right)}),
                    }
                },
                Operator::LessEqual => {
                    match left_number.zip(right_number).map(|(a,b)| a <= b) {
                        Some(result) => Ok(Value::Boolean(result)),
                        None => Err(RuntimeError{message:format!("Comparison error: <= {:?} {:?}", left, right)}),
                    }
                },
                Operator::Less => {
                    match left_number.zip(right_number).map(|(a,b)| a < b) {
                        Some(result) => Ok(Value::Boolean(result)),
                        None => Err(RuntimeError{message:format!("Comparison error: < {:?} {:?}", left, right)}),
                    }
                },
                _ => todo!(),
            }
        },
        _ =>{ 
            println!("eval3 {:?}", expr);
            todo!()
        }
    }
}
