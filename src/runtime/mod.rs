// Runtime tools for script engine
mod scope;
mod function;
mod core;
mod callable;
mod hash;

pub use self::scope::*;
pub use self::function::*;
pub use self::core::core_func_call;
pub use self::callable::*;
pub use self::hash::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use error::Error;
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

    // Pair
    Pair(Ref< Value >, Ref< Value >),

    // Reference types
    Str(Ref< String >),
    List(Ref< Vec<Value> >),
    Obj(Ref< HashMap<String,Value> >),
    Map(Ref< HashMap<HashV,(Value,Value)> >),

    // Callable reference types
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
        use self::Value::*;
        match *self {
            Val(ref v) => write!(f, "{}", v),
            Ref(ref v) => write!(f, "{}", v.borrow()),
            Pair(ref n, ref m) => {
                let n = n.borrow();
                let m = m.borrow();
                write!(f, "<{}, {}>", n, m)
            },
            Str(ref s) => {
                let s = s.borrow();
                write!(f, "\"{}\"", s)
            },
            List(ref l) => {
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
            Obj(ref o) => {
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
            Map(ref m) => {
                let m = m.borrow();
                write!(f, "map{{")?;
                if m.len() > 0 {
                    for (_,(k,v)) in m.iter().take(m.len()-1) {
                        write!(f, "[{}]= {}, ", k, v)?;
                    }
                    let (_,(k,v)) = m.iter().skip(m.len()-1).next().unwrap();
                    write!(f, "[{}]= {}", k, v)?;
                }
                write!(f, "}}")
            },
            Func(ref p, ref n) => {
                let p = p.borrow();
                let n = n.borrow();
                write!(f, "function{{{}::{}}}", p, n)
            },
            // TODO: make this a bit more verbose
            Closure(_,_) => write!(f, "closure{{}}"),
            Null => write!(f, "null"),
        }
    }
}

impl fmt::Display for VType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::VType::*;
        match *self {
            I(n) => write!(f, "{}", n),
            F(n) => write!(f, "{}", n),
            B(b) => write!(f, "{}", b),
        }
    }
}

// Runtime Signals
pub enum Signal {
    Error(Error),
    Return(Value),
    Continue,
    Break,
    // Exception,
    Done,
}

pub type ExprRes = Result<Value, Error>;
