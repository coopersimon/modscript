use super::{Value, Signal, ExprRes};
use error::{mserr, Error, Type, RunCode, CriticalCode};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Scope {
    vars: Vec<HashMap<String, Value>>,
}


impl Scope {
    pub fn new() -> Self {
        Scope {
            vars: vec![HashMap::new()],
        }
    }

    pub fn extend(&mut self) {
        self.vars.push(HashMap::new());
    }

    pub fn reduce(&mut self) {
        self.vars.pop();
    }

    pub fn new_var(&mut self, name: &str, val: Value) -> Signal {
        match self.vars.last_mut() {
            Some(t) => match t.contains_key(name) {
                true => Signal::Error(Error::new(Type::RunTime(RunCode::VariableAlreadyDeclared))),
                false => {t.insert(name.to_string(), val); Signal::Done},
            },
            // critical error
            None => Signal::Error(Error::new(Type::Critical(CriticalCode::ScopeError))),
        }
    }

    pub fn get_var(&self, name: &str) -> ExprRes {
        use Value::*;
        for t in self.vars.iter().rev() {
            match t.get(name) {
                Some(Ref(ref v)) => return Ok(Val(v.borrow().clone())),
                Some(v) => return Ok(v.clone()),
                None => {},
            }
        }

        mserr(Type::RunTime(RunCode::VariableNotDeclared))
    }

    // may create reference
    pub fn get_ref(&mut self, name: &str) -> ExprRes {
        use Value::*;
        for t in self.vars.iter_mut().rev() {
            let mut make_ref = false;
            let out = match t.get(name) {
                Some(Val(v)) => {
                    make_ref = true;
                    Some(Ref(Rc::new(RefCell::new(v.clone()))))
                },
                Some(v) => Some(v.clone()),
                None => None,
            };
            match out {
                None => {},
                Some(v) => {
                    if make_ref {
                        t.insert(name.to_string(), v.clone());
                    }
                    return Ok(v);
                },
            }
        }

        mserr(Type::RunTime(RunCode::VariableNotDeclared))
    }

    pub fn set_var(&mut self, name: &str, val: Value) -> Signal {
        use Value::*;
        for t in self.vars.iter_mut().rev() {
            match t.get_mut(name) {
                Some(v) => match val {
                    Val(ref vtype_val) => match v {
                        Ref(v_ref) => {*v_ref.borrow_mut() = vtype_val.clone(); return Signal::Done},
                        _ => {*v = val.clone(); return Signal::Done},
                    },
                    _ => {*v = val.clone(); return Signal::Done},
                },
                None => {},
            }
        }

        Signal::Error(Error::new(Type::RunTime(RunCode::VariableNotDeclared)))
    }

    // For closures
    pub fn get_scope_refs(&mut self) -> Vec<(String, Value)> {
        use Value::*;
        let mut scope = HashMap::new();
        for t in self.vars.iter_mut() {
            for (k,v) in t.iter_mut() {
                let value = match v {
                    Val(val) => Some(val.clone()),
                    _ => None,
                };
                /*match val {
                    Some(val) => {
                        *v = Ref(Rc::new(RefCell::new(val)));
                        out.insert(k.clone(), v.clone());
                    },
                    None => out.insert(k.clone(), v.clone()),
                }*/
                if let Some(val) = value {
                    *v = Ref(Rc::new(RefCell::new(val)));
                }
                scope.insert(k.clone(), v.clone());
            }
        }
        let out = scope.drain().collect::<Vec<(String, Value)>>();
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use VType::*;

    fn is_error(s: Signal) -> bool {
        match s {
            Signal::Error(_) => true,
            _ => false,
        }
    }

    fn is_expr_error(s: ExprRes) -> bool {
        match s {
            Err(_) => true,
            _ => false,
        }
    }

    // BASIC TESTS
    #[test]
    fn declare_variable() {
        let mut state = Scope::new();

        assert_eq!(state.new_var("x", Value::Val(I(30))), Signal::Done);
    }

    #[test]
    fn read_variable() {
        let mut state = Scope::new();

        state.new_var("x", Value::Val(I(30)));

        assert_eq!(state.get_var("x"), Ok(Value::Val(I(30))));
    }

    #[test]
    fn read_undeclared_variable() {
        let state = Scope::new();

        assert!(is_expr_error(state.get_var("x")));
    }

    #[test]
    fn set_undeclared_variable() {
        let mut state = Scope::new();

        assert!(is_error(state.set_var("x", Value::Val(I(30)))));
    }

    #[test]
    fn set_variable() {
        let mut state = Scope::new();

        state.new_var("x", Value::Val(I(30)));

        assert_eq!(state.get_var("x"), Ok(Value::Val(I(30))));

        state.set_var("x", Value::Val(F(2.5)));

        assert_eq!(state.get_var("x"), Ok(Value::Val(F(2.5))));
    }

    #[test]
    fn set_multi_variables() {
        let mut state = Scope::new();

        state.new_var("x", Value::Val(I(30)));

        state.new_var("y", Some(Value::Val(F(3.3))));

        assert_eq!(state.get_var("x"), Ok(Value::Val(I(30))));

        assert_eq!(state.get_var("y"), Ok(Value::Val(F(3.3))));
    }

    // SCOPE TESTS
    #[test]
    fn extend_scope() {
        let mut state = Scope::new();

        state.extend();

        state.new_var("x", Value::Val(I(30)));

        assert_eq!(state.get_var("x"), Ok(Value::Val(I(30))));

        state.reduce();

        assert!(is_expr_error(state.get_var("x")));
    }

    #[test]
    fn shadow_variables() {
        let mut state = Scope::new();

        state.new_var("x", Value::Val(I(30)));

        assert_eq!(state.get_var("x"), Ok(Value::Val(I(30))));

        state.extend();

        state.new_var("x", Some(Value::Val(F(2.5))));

        assert_eq!(state.get_var("x"), Ok(Value::Val(F(2.5))));
    }

    #[test]
    fn shadow_variables_and_retract() {
        let mut state = Scope::new();

        state.new_var("x", Value::Val(I(30)));

        assert_eq!(state.get_var("x"), Ok(Value::Val(I(30))));

        state.extend();

        state.new_var("x", Value::Val(F(2.5)));

        assert_eq!(state.get_var("x"), Ok(Value::Val(F(2.5))));

        state.reduce();

        assert_eq!(state.get_var("x"), Ok(Value::Val(I(30))));
    }
}
