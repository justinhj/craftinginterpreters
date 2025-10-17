# Crafting Interpreters

Various things related to the book [Crafting Interpreters](https://github.com/munificent/craftinginterpreters)

## Current progress

Last thing implemented was call function. See:

git show cbb0b98

+        Call(callee, arguments) => eval_call(callee, arguments, Rc::clone(&eval_state)),
+fn eval_call(callee: &Expr, arguments: &[Expr], eval_state: Rc<RefCell<EvalState>>) -> Result<Value, RuntimeError> {
+    let callee_evaluated = eval_expression(callee,Rc::clone(&eval_state));
+
in the parser

iff --git a/chapter8-13/src/parse.rs b/chapter8-13/src/parse.rs
index fe3d73f..4b48ec9 100644
--- a/chapter8-13/src/parse.rs
+++ b/chapter8-13/src/parse.rs
@@ -8,6 +8,7 @@ pub enum Value {
     Boolean(bool),
     Number(f64),
     Nil,
+    Callable(Box<Value>,Vec<Value>),
 }

 #[derive(Debug)]
@@ -123,6 +124,7 @@ impl Display for Value {
                 }
             }
             Value::Nil => write!(f, "nil"),
+            Value::Callable(callee,args) => write!(f, "Call some {} with {:?}", callee, args),
         }
     }
 }

somewhere here

https://craftinginterpreters.com/functions.html#interpreting-function-calls




