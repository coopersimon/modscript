// Runtime tools for script engine
mod scope;
mod function;
mod core;

pub use self::scope::*;
pub use self::function::*;
pub use self::core::core_func_call;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

use ast::FuncRoot;

use std::fmt;

pub type Ref<T> = Rc<RefCell<T>>;

// Types
#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    // Value type
    Val(VType),

    // Reference value type
    Ref(Ref< VType >),

    // Reference types
    Str(Ref< String >),
    List(Ref< Vec<Value> >),
    Obj(Ref< BTreeMap<String,Value> >),
    Func(Ref< String >, Ref< String >),
    Closure(Ref< FuncRoot >, Ref< Vec<(String,Value)> >),

    // Null
    Null,
}

// Value types
#[derive(Clone, PartialEq, Debug)]
pub enum VType {
    I(i64),
    F(f64),
    B(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Val(ref v) => write!(f, "{}", v),
            &Value::Ref(ref v) => write!(f, "{}", v.borrow()),
            &Value::Str(ref s) => {
                let s = s.borrow();
                write!(f, "\"{}\"", s)
            },
            &Value::List(ref l) => {
                let l = l.borrow();
                write!(f, "[")?;
                if l.len() > 0 {
                    for e in l.iter().take(l.len()-1) {
                        write!(f, "{}, ", e)?;
                    }
                    write!(f, "{}", l.last().unwrap())?;
                }
                write!(f, "]")
            },
            &Value::Obj(ref o) => {
                let o = o.borrow();
                write!(f, "object{{")?;
                if o.len() > 0 {
                    for (k,v) in o.iter().take(o.len()-1) {
                        write!(f, "{}: {}, ", k, v)?;
                    }
                    let (k,v) = o.iter().skip(o.len()-1).next().unwrap();
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            },
            &Value::Func(ref p, ref n) => {
                let p = p.borrow();
                let n = n.borrow();
                write!(f, "function{{{}::{}}}", p, n)
            },
            // TODO: make this a bit more verbose
            &Value::Closure(_,_) => write!(f, "closure{{}}"),
            &Value::Null => write!(f, "null"),
        }
    }
}

impl fmt::Display for VType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &VType::I(n) => write!(f, "{}", n),
            &VType::F(n) => write!(f, "{}", n),
            &VType::B(b) => write!(f, "{}", b),
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
