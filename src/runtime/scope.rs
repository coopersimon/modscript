use super::{Value, Signal, ExprRes, expr_err};
use std::collections::HashMap;

enum Stored {
    Const(Value),
    Var(Value),
}

pub struct Scope {
    vars: Vec<HashMap<String, Stored>>,
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
                true => Signal::Error("Value already declared.".to_string()),
                false => {t.insert(name.to_string(), Stored::Var(val)); Signal::Done},
            },
            // critical error
            None => Signal::Error("Internal critical scope error.".to_string()),
        }
    }

    pub fn new_const(&mut self, name: &str, val: Value) -> Signal {
        match self.vars.last_mut() {
            Some(t) => match t.contains_key(name) {
                true => Signal::Error("Value already declared.".to_string()),
                false => {t.insert(name.to_string(), Stored::Const(val)); Signal::Done},
            },
            // critical error
            None => Signal::Error("Internal critical scope error.".to_string()),
        }
    }

    pub fn get_var(&self, name: &str) -> ExprRes {
        for t in self.vars.iter().rev() {
            match t.get(name) {
                Some(Stored::Const(v)) => return Ok(v.clone()),
                Some(Stored::Var(v)) => return Ok(v.clone()),
                None => {},
            }
        }

        expr_err("Value not declared.")
    }

    pub fn set_var(&mut self, name: &str, val: Value) -> Signal {
        for t in self.vars.iter_mut().rev() {
            /*match t.contains_key(name) {
                true => {t.insert(name.to_string(), val); return Signal::Done},
                false => {},
            }*/
            match t.get(name) {
                Some(Stored::Const(_)) => return Signal::Error("Cannot redefine constant.".to_string()),
                Some(Stored::Var(_)) => {t.insert(name.to_string(), Stored::Var(val)); return Signal::Done},
                None => {},
            }
        }

        Signal::Error("Value not declared.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(state.new_var("x", Value::Int(30)), Signal::Done);
    }

    #[test]
    fn read_variable() {
        let mut state = Scope::new();

        state.new_var("x", Value::Int(30));

        assert_eq!(state.get_var("x"), Ok(Value::Int(30)));
    }

    #[test]
    fn read_undeclared_variable() {
        let state = Scope::new();

        assert!(is_expr_error(state.get_var("x")));
    }

    #[test]
    fn set_undeclared_variable() {
        let mut state = Scope::new();
        
        assert!(is_error(state.set_var("x", Value::Int(30))));
    }

    #[test]
    fn set_variable() {
        let mut state = Scope::new();
        
        state.new_var("x", Value::Int(30));

        assert_eq!(state.get_var("x"), Ok(Value::Int(30)));

        state.set_var("x", Value::Float(2.5));

        assert_eq!(state.get_var("x"), Ok(Value::Float(2.5)));
    }

    #[test]
    fn set_multi_variables() {
        let mut state = Scope::new();
        
        state.new_var("x", Value::Int(30));

        state.new_var("y", Some(Value::Float(3.3)));
        
        assert_eq!(state.get_var("x"), Ok(Value::Int(30)));

        assert_eq!(state.get_var("y"), Ok(Value::Float(3.3)));
    }

    // SCOPE TESTS
    #[test]
    fn extend_scope() {
        let mut state = Scope::new();

        state.extend();
        
        state.new_var("x", Value::Int(30));

        assert_eq!(state.get_var("x"), Ok(Value::Int(30)));

        state.reduce();

        assert!(is_expr_error(state.get_var("x")));
    }

    #[test]
    fn shadow_variables() {
        let mut state = Scope::new();

        state.new_var("x", Value::Int(30));

        assert_eq!(state.get_var("x"), Ok(Value::Int(30)));

        state.extend();
        
        state.new_var("x", Some(Value::Float(2.5)));

        assert_eq!(state.get_var("x"), Ok(Value::Float(2.5)));
    }

    #[test]
    fn shadow_variables_and_retract() {
        let mut state = Scope::new();

        state.new_var("x", Value::Int(30));

        assert_eq!(state.get_var("x"), Ok(Value::Int(30)));

        state.extend();
        
        state.new_var("x", Value::Float(2.5));

        assert_eq!(state.get_var("x"), Ok(Value::Float(2.5)));

        state.reduce();

        assert_eq!(state.get_var("x"), Ok(Value::Int(30)));
    }
}
