use super::{AstNode, Expr, Assign};
use runtime::{Value, VType, Scope, Signal, FuncMap};

pub struct IndexAssign {
    index: Box<Expr>,
    child_op: Option<Box<Assign>>,
}

pub struct AccessAssign {
    field_name: String,
    child_op: Option<Box<Assign>>,
}

// IMPLS

impl IndexAssign {
    pub fn new(i: Box<Expr>, c: Option<Box<Assign>>) -> Self {
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
            Ok(Val(I(i))) => i,
            Ok(_) => return Signal::Error("Cannot access with non-index.".to_string()),
            Err(e) => return Signal::Error(e),
        };

        match var {
            List(ref l) => {
                let mut list = l.borrow_mut();

                let index = if (i >= 0) && ((i as usize) < list.len()) {
                    i as usize
                } else if (i < 0) && ((i.abs() as usize) <= list.len()) {
                    ((list.len() as i64) + i) as usize
                } else {
                    return Signal::Error("Index access out of bounds in assign.".to_string())
                };

                match self.child_op {
                    Some(ref op) => op.assign(list[index].clone(), val, state, f),
                    None => {list[index] = val; Signal::Done},
                }
            },
            _ => Signal::Error("Cannot index non-list type.".to_string()),
        }
    }
}


impl AccessAssign {
    pub fn new(f: &str, c: Option<Box<Assign>>) -> Self {
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
                    None => return Signal::Error("Field in object assignment doesn't exist.".to_string()),
                };

                match self.child_op {
                    Some(ref op) => op.assign(field.clone(), val, state, f),
                    None => {*field = val; Signal::Done},
                }
            },
            _ => Signal::Error("Cannot index non-list type.".to_string()),
        }
    }
}
