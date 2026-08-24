use crate::env::Environment;
use crate::eval::Expr::{Assign, Binary, Call, Grouping, Literal, Logical, Unary, Variable};
use crate::parse::Operator;
use crate::parse::{Expr, Stmt, Value};
use std::fmt;

#[derive(Debug)]
pub struct RuntimeError(pub String);

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime error: {}", self.0)
    }
}

// All values have a true or false value. The only things that are false in lox are nil and
// boolean false, everything else is true
fn bool_value(value: &Value) -> bool {
    !(matches!(value, Value::Boolean(false)) || matches!(value, Value::Nil))
}

fn numeric_value(value: &Value) -> Option<f64> {
    match value {
        Value::Number(num) => Some(*num),
        _ => None,
    }
}

pub type EvalResult = Result<Value, RuntimeError>;

pub fn eval_statements(stmts: &[Stmt], environment: &mut Environment) -> Result<(), RuntimeError> {
    for stmt in stmts {
        match stmt {
            Stmt::VarDecl(id, Some(expr)) => {
                let value = eval_expression(expr, environment)?;
                environment.define(id.to_string(), Some(value));
            }
            Stmt::VarDecl(id, None) => {
                environment.define(id.to_string(), None);
            }
            Stmt::Block(stmts) => {
                let previous_block = environment.push_scope();
                eval_statements(stmts, environment)?;
                environment.pop_scope(previous_block);
            }
            // Print can become a builtin native
            Stmt::Print(expr) => {
                let value = eval_expression(expr, environment)?;
                println!("{}", value);
            }
            Stmt::Expression(expr) => match eval_expression(expr, environment) {
                Ok(_) => (),
                Err(err) => return Err(err),
            },
            Stmt::If(expr, then_stmt, else_stmt) => {
                let cond = eval_expression(expr, environment)?;
                let cond_bool = bool_value(&cond);
                if cond_bool {
                    eval_statements(then_stmt, environment)?
                } else {
                    eval_statements(else_stmt, environment)?
                }
            }
            Stmt::While(expr, stmts) => loop {
                let cond = eval_expression(expr, environment)?;
                let cond_bool = bool_value(&cond);
                if cond_bool {
                    eval_statements(stmts, environment)?
                } else {
                    break;
                }
            },
        }
    }
    Ok(())
}

#[rustfmt::skip]
pub fn eval_expression(expr: &Expr, environment: &mut Environment) -> EvalResult {
    match expr {
        Literal(value) => Ok(value.clone()),
        Call(callee, arguments) => eval_call(callee, arguments, environment),
        Unary(operator, right) => {
            let right = eval_expression(right, environment)?;
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
            let left = eval_expression(left, environment)?;
            let right = eval_expression(right, environment)?;
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
            let left = eval_expression(left, environment)?;
            match operator {
                Operator::And if !bool_value(&left) => Ok(left),
                Operator::Or if bool_value(&left) => Ok(left),
                Operator::Or | Operator::And => {
                    eval_expression(right, environment)
                },
                _ => Err(RuntimeError(format!("Unexpected logical operator : {}", operator)))
            }
        },
        Grouping(expr) => eval_expression(expr, environment),
        Variable(id) => {
            match environment.lookup(id) {
                Ok(value) => Ok(value),
                err @ Err(_) => err,
            }
        },
        Assign(id, expr) => {
            let value = eval_expression(expr, environment)?;
            environment.assign(id, value)
        },
    }
}

fn eval_call(
    callee: &Expr,
    arguments: &[Expr],
    environment: &mut Environment,
) -> Result<Value, RuntimeError> {
    let function = eval_expression(callee, environment)?;

    let (name, expected_arity) = match &function {
        Value::Function { name, params, .. } => (name.as_str(), params.len()),
        Value::NativeFunction { name, arity, .. } => (name.as_str(), *arity),
        _ => {
            return Err(RuntimeError(
                "Can only call functions and classes.".to_string(),
            ));
        }
    };

    if arguments.len() != expected_arity {
        return Err(RuntimeError(format!(
            "Function `{}` expected {} arguments but got {}.",
            name,
            expected_arity,
            arguments.len()
        )));
    }

    let mut evaluated_args = vec![];
    for arg in arguments {
        evaluated_args.push(eval_expression(arg, environment)?);
    }

    match function {
        Value::Function {
            name,
            params,
            body,
            closure_env,
        } => {
            let prev_env = environment.push_scope();
            for (param, val) in params.iter().zip(evaluated_args) {
                environment.define(param.clone(), Some(val));
            }
            let result = eval_statements(&body, environment);
            environment.pop_scope(prev_env);
            result?; // later chapter
            Ok(Value::Nil)
        }
        Value::NativeFunction { name, .. } if name == "clock" => {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs_f64())
                .unwrap_or(0.0);
            Ok(Value::Number(now))
        }
        _ => unreachable!(),
    }
}

// Nil is only equal to nil
// Two numbers can be compared
// Two bools can be compared
// Otherwise it is not equal
// TODO does it make sense to compare function objects?
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
            )));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse;
    use crate::scan::scan;

    fn run(source: &str) -> (Result<(), RuntimeError>, Environment) {
        let tokens = scan(source).unwrap();
        let stmts = parse(&tokens).unwrap();
        let mut env = Environment::new();
        let result = eval_statements(&stmts, &mut env);
        (result, env)
    }

    fn run_expr(source: &str) -> Value {
        let tokens = scan(source).unwrap();
        let stmts = parse(&tokens).unwrap();
        let mut env = Environment::new();
        match &stmts[0] {
            crate::parse::Stmt::Expression(expr) => eval_expression(expr, &mut env).unwrap(),
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn eval_number_literal() {
        let value = run_expr("42;");
        assert_eq!(value, Value::Number(42.0));
    }

    #[test]
    fn eval_string_literal() {
        let value = run_expr("\"hello\";");
        assert_eq!(value, Value::String("hello".to_string()));
    }

    #[test]
    fn eval_boolean_literals() {
        assert_eq!(run_expr("true;"), Value::Boolean(true));
        assert_eq!(run_expr("false;"), Value::Boolean(false));
    }

    #[test]
    fn eval_nil_literal() {
        assert_eq!(run_expr("nil;"), Value::Nil);
    }

    #[test]
    fn eval_arithmetic() {
        assert_eq!(run_expr("1 + 2;"), Value::Number(3.0));
        assert_eq!(run_expr("10 - 3;"), Value::Number(7.0));
        assert_eq!(run_expr("3 * 4;"), Value::Number(12.0));
        assert_eq!(run_expr("10 / 2;"), Value::Number(5.0));
    }

    #[test]
    fn eval_operator_precedence() {
        assert_eq!(run_expr("2 + 3 * 4;"), Value::Number(14.0));
        assert_eq!(run_expr("(2 + 3) * 4;"), Value::Number(20.0));
    }

    #[test]
    fn eval_unary_negation() {
        assert_eq!(run_expr("-5;"), Value::Number(-5.0));
        assert_eq!(run_expr("--5;"), Value::Number(5.0));
    }

    #[test]
    fn eval_unary_bang() {
        assert_eq!(run_expr("!true;"), Value::Boolean(false));
        assert_eq!(run_expr("!false;"), Value::Boolean(true));
        assert_eq!(run_expr("!nil;"), Value::Boolean(true));
        assert_eq!(run_expr("!1;"), Value::Boolean(false));
    }

    #[test]
    fn eval_comparison() {
        assert_eq!(run_expr("1 < 2;"), Value::Boolean(true));
        assert_eq!(run_expr("2 < 1;"), Value::Boolean(false));
        assert_eq!(run_expr("1 <= 1;"), Value::Boolean(true));
        assert_eq!(run_expr("2 > 1;"), Value::Boolean(true));
        assert_eq!(run_expr("1 >= 1;"), Value::Boolean(true));
    }

    #[test]
    fn eval_equality() {
        assert_eq!(run_expr("1 == 1;"), Value::Boolean(true));
        assert_eq!(run_expr("1 != 2;"), Value::Boolean(true));
        assert_eq!(run_expr("true == true;"), Value::Boolean(true));
        assert_eq!(run_expr("nil == nil;"), Value::Boolean(true));
        assert_eq!(run_expr("\"a\" == \"a\";"), Value::Boolean(true));
        assert_eq!(run_expr("\"a\" != \"b\";"), Value::Boolean(true));
    }

    #[test]
    fn eval_string_concatenation() {
        assert_eq!(
            run_expr("\"hello\" + \" world\";"),
            Value::String("hello world".to_string())
        );
    }

    #[test]
    fn eval_var_declaration_and_lookup() {
        let (result, env) = run("var x = 10;");
        assert!(result.is_ok());
        assert_eq!(env.lookup("x").unwrap(), Value::Number(10.0));
    }

    #[test]
    fn eval_var_assignment() {
        let (result, env) = run("var x = 1; x = 42;");
        assert!(result.is_ok());
        assert_eq!(env.lookup("x").unwrap(), Value::Number(42.0));
    }

    #[test]
    fn eval_block_scoping() {
        let (result, env) = run("var x = 1; { var x = 2; } ");
        assert!(result.is_ok());
        assert_eq!(env.lookup("x").unwrap(), Value::Number(1.0));
    }

    #[test]
    fn eval_block_sees_outer_variable() {
        let (result, env) = run("var x = 1; { x = 99; }");
        assert!(result.is_ok());
        assert_eq!(env.lookup("x").unwrap(), Value::Number(99.0));
    }

    #[test]
    fn eval_while_loop() {
        let (result, env) = run("var x = 0; while (x < 5) { x = x + 1; }");
        assert!(result.is_ok());
        assert_eq!(env.lookup("x").unwrap(), Value::Number(5.0));
    }

    #[test]
    fn eval_if_true_branch() {
        let (result, env) = run("var x = 0; if (true) { x = 1; } else { x = 2; }");
        assert!(result.is_ok());
        assert_eq!(env.lookup("x").unwrap(), Value::Number(1.0));
    }

    #[test]
    fn eval_if_false_branch() {
        let (result, env) = run("var x = 0; if (false) { x = 1; } else { x = 2; }");
        assert!(result.is_ok());
        assert_eq!(env.lookup("x").unwrap(), Value::Number(2.0));
    }

    #[test]
    fn eval_logical_and() {
        assert_eq!(run_expr("true and true;"), Value::Boolean(true));
        assert_eq!(run_expr("true and false;"), Value::Boolean(false));
        assert_eq!(run_expr("false and true;"), Value::Boolean(false));
    }

    #[test]
    fn eval_logical_or() {
        assert_eq!(run_expr("false or true;"), Value::Boolean(true));
        assert_eq!(run_expr("false or false;"), Value::Boolean(false));
        assert_eq!(run_expr("true or false;"), Value::Boolean(true));
    }

    #[test]
    fn eval_logical_short_circuit() {
        let (result, env) = run("var x = 1; false and (x = 2);");
        assert!(result.is_ok());
        assert_eq!(env.lookup("x").unwrap(), Value::Number(1.0));
    }

    #[test]
    fn eval_uninitialized_variable_errors() {
        let (result, _) = run("var x; x = x + 1;");
        assert!(result.is_err());
    }

    #[test]
    fn eval_undefined_variable_errors() {
        let (result, _) = run("y = 1;");
        assert!(result.is_err());
    }

    #[test]
    fn eval_division() {
        assert_eq!(run_expr("10 / 3;"), Value::Number(10.0 / 3.0));
    }

    #[test]
    fn eval_nested_while() {
        let (result, env) = run(
            "var sum = 0; var i = 0; while (i < 3) { var j = 0; while (j < 3) { sum = sum + 1; j = j + 1; } i = i + 1; }",
        );
        assert!(result.is_ok());
        assert_eq!(env.lookup("sum").unwrap(), Value::Number(9.0));
    }
}
