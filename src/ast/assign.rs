use super::{AstNode, Expr, Assign};
use runtime::{Value, VType, Scope, Signal, FuncMap, hash_value};
use error::{Error, Type, RunCode};

pub struct IndexAssign {
    index: Box<dyn Expr>,
    child_op: Option<Box<dyn Assign>>,
}

pub struct AccessAssign {
    field_name: String,
    child_op: Option<Box<dyn Assign>>,
}

// IMPLS

impl IndexAssign {
    pub fn new(i: Box<dyn Expr>, c: Option<Box<dyn Assign>>) -> Self {
        IndexAssign {
            index: i,
            child_op: c,
        }
    }
}

impl AstNode for IndexAssign {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Assign for IndexAssign {
    fn assign(&self, var: Value, val: Value, state: &mut Scope, f: &FuncMap) -> Signal {
        use Value::*;
        use self::VType::*;

        let i = match self.index.eval(state, f) {
            Ok(i)   => i,
            Err(e)  => return Signal::Error(e),
        };

        match var {
            List(ref l) => {
                let mut list = l.borrow_mut();

                let i = match i {
                    Val(I(i)) => i,
                    _ => return Signal::Error(Error::new(Type::RunTime(RunCode::TypeError))),
                };

                let index = if (i >= 0) && ((i as usize) < list.len()) {
                    i as usize
                } else if (i < 0) && ((i.abs() as usize) <= list.len()) {
                    ((list.len() as i64) + i) as usize
                } else {
                    return Signal::Error(Error::new(Type::RunTime(RunCode::OutOfBounds)));
                };

                match self.child_op {
                    Some(ref op) => op.assign(list[index].clone(), val, state, f),
                    None => {list[index] = val; Signal::Done},
                }
            },
            Map(ref m) => {
                let mut map = m.borrow_mut();

                let index = match hash_value(&i) {
                    Ok(i) => i,
                    Err(e) => return Signal::Error(e),
                };
                let map_value = match map.get_mut(&index) {
                    Some(v) => v,
                    None    => return Signal::Error(Error::new(Type::RunTime(RunCode::OutOfBounds))),
                };

                match self.child_op {
                    Some(ref op) => op.assign(map_value.1.clone(), val, state, f),
                    None => {map_value.1 = val; Signal::Done},
                }
            },
            _ => Signal::Error(Error::new(Type::RunTime(RunCode::TypeError))),
        }
    }
}


impl AccessAssign {
    pub fn new(f: &str, c: Option<Box<dyn Assign>>) -> Self {
        AccessAssign {
            field_name: f.to_string(),
            child_op: c,
        }
    }
}

impl AstNode for AccessAssign {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Assign for AccessAssign {
    fn assign(&self, var: Value, val: Value, state: &mut Scope, f: &FuncMap) -> Signal {
        use Value::*;

        match var {
            Obj(ref o) => {
                let mut object = o.borrow_mut();

                let field = match object.get_mut(&self.field_name) {
                    Some(f) => f,
                    None => return Signal::Error(Error::new(Type::RunTime(RunCode::FieldNotFound))),
                };

                match self.child_op {
                    Some(ref op) => op.assign(field.clone(), val, state, f),
                    None => {*field = val; Signal::Done},
                }
            },
            _ => Signal::Error(Error::new(Type::RunTime(RunCode::TypeError))),
        }
    }
}
