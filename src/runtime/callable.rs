// wrapper around Value making it callable
use super::{Value, ExprRes, FuncMap};
use error::{mserr, Type, RunCode};

pub struct Callable {
    base: Value,
}

impl Callable {
    pub fn new(b: Option<Value>) -> Self {
        Callable {
            base: match b {
                Some(b_in) => b_in,
                None => Value::Null,
            },
        }
    }

    pub fn call(&self, f: &FuncMap, args: &[Value]) -> ExprRes {
        match self.base {
            Value::Func(ref package, ref name) => f.call_fn(&package.borrow(), &name.borrow(), args),
            Value::Closure(ref func, _) => func.borrow().call(args, f, None),
            Value::Null => Ok(Value::Null),
            _ => mserr(Type::RunTime(RunCode::InvalidCall)),
        }
    }

    pub fn set_value(&mut self, val: Value) {
        self.base = val;
    }
}
