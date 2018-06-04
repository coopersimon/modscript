// Runtime tools for script engine
mod scope;
mod function;

pub use self::scope::*;
pub use self::function::*;

use std::rc::Rc;
use std::cell::RefCell;

use std::fmt;

pub type Ref<T> = Rc<RefCell<T>>;

// Types
#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String), // TODO: switch to ref?
    List(Ref< Vec<Value> >),
    //
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Int(n) => write!(f, "{}", n),
            &Value::Float(n) => write!(f, "{}", n),
            &Value::Str(ref s) => write!(f, "\"{}\"", s),
            &Value::Bool(b) => write!(f, "{}", b),
            &Value::List(ref l) => {
                let l = l.borrow();
                write!(f, "[")?;
                for e in l.iter() {
                    write!(f, "{}, ", e)?;
                }
                write!(f, "]")
            },
            &Value::Null => write!(f, "null"),
        }
    }
}

// Runtime Signals
#[derive(Clone, PartialEq, Debug)]
pub enum Signal {
    Error(String),
    Return(Value),
    Continue,
    Break,
    // Exception,
    Done,
}

pub type ExprRes = Result<Value, String>;

pub fn expr_err(err: &str) -> ExprRes {
    Err(err.to_string())
}
