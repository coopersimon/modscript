// Runtime tools for script engine
mod state;
mod function;

pub use self::state::*;
pub use self::function::*;

use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Null,
    // List
    // Object
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Int(n) => write!(f, "{}", n),
            &Value::Float(n) => write!(f, "{}", n),
            &Value::Str(ref s) => write!(f, "\"{}\"", s),
            &Value::Bool(b) => write!(f, "{}", b),
            &Value::Null => write!(f, "null"),
        }
    }
}

// Runtime Signals
#[derive(Clone, PartialEq, Debug)]
pub enum Signal {
    Error(String),
    Return(Value),
    Done,
    //Continue
    //Break
}

pub type ExprRes = Result<Value, String>;

pub fn expr_err(err: &str) -> ExprRes {
    Err(err.to_string())
}
