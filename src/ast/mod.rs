// scripting engine ast
mod stat;
mod expr;

pub use self::stat::*;
pub use self::expr::*;

use runtime::{Value, Scope, ExprRes, expr_err, Signal, FuncMap, PackageRoot};

use std::collections::BTreeMap;

pub trait AstNode {
    fn print(&self) -> String;
    // compile
}

pub trait Expr: AstNode {
    fn eval(&self, &mut Scope, &FuncMap) -> ExprRes;
}

pub trait Statement: AstNode {
    fn run(&self, &mut Scope, &FuncMap) -> Signal;
}

// AST entry point for script snippet
pub struct Script {
    stat: Box<Statement>,
}

impl Script {
    pub fn new(s: Box<Statement>) -> Self {
        Script {
            stat: s,
        }
    }

    pub fn run(&self, funcs: &FuncMap) -> Signal {
        let mut state = Scope::new();

        self.stat.run(&mut state, funcs)
    }
}


// For packages of functions
pub struct ScriptPackage {
    pub funcs: BTreeMap<String, FuncRoot>,
}

impl ScriptPackage {
    pub fn new(f: BTreeMap<String, FuncRoot>) -> Self {
        ScriptPackage {
            funcs: f,
        }
    }

    pub fn call_ref(self) -> PackageRoot {
        Box::new(move |n, a, f| {
            match self.funcs.get(n) {
                Some(func) => func.call(a, f),
                None => Err(format!("Couldn't find function {}.", n)),
            }
        })
    }
}


// AST entry point for function
pub struct FuncRoot {
    arg_names: Vec<String>,
    stat_list: Vec<Box<Statement>>,
}

impl FuncRoot {
    pub fn new(arg_names: Vec<String>, stat_list: Vec<Box<Statement>>) -> Self {
        FuncRoot {
            /*arg_names: match arg_names {
                Some(v) => v,
                None => Vec::new(),
            },*/
            arg_names: arg_names,
            stat_list: stat_list,
        }
    }

    pub fn call(&self, args: &[Value], f: &FuncMap) -> ExprRes {
        let mut state = Scope::new();

        if args.len() != self.arg_names.len() {
            return expr_err("Incorrect number of arguments provided.");
        }

        for (a,n) in args.iter().zip(self.arg_names.iter()) {
            state.new_var(&n, a.clone());
        }

        for s in &self.stat_list {
            match s.run(&mut state, f) {
                Signal::Done => {},
                Signal::Error(e) => return Err(e),
                Signal::Return(v) => return Ok(v),
                //Continue => return Err(...),
                //Break
            }
        }

        Ok(Value::Null)
    }

    pub fn get_arg_names(&self) -> &[String] {
        self.arg_names.as_slice()
    }
}

impl AstNode for FuncRoot {
    fn print(&self) -> String {
        "var".to_string()
    }
}
