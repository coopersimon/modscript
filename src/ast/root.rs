use super::{AstNode, Expr, Statement};
use runtime::{Value, Scope, Signal, ExprRes, FuncMap, PackageRoot};
use error::{mserr, Error, Type, RunCode};

use std::collections::BTreeMap;
use std::{cmp, fmt};

// AST entry point for statement snippet
pub struct Script {
    stat: Box<dyn Statement>,
}

impl Script {
    pub fn new(s: Box<dyn Statement>) -> Self {
        Script {
            stat: s,
        }
    }

    pub fn run(&self, funcs: &FuncMap) -> Signal {
        let mut state = Scope::new();

        self.stat.run(&mut state, funcs)
    }

    pub fn repl_run(&self, state: &mut Scope, funcs: &FuncMap) -> Signal {
        self.stat.run(state, funcs)
    }
}


// AST entry point for expression snippet
pub struct ScriptExpr {
    expr: Option<Box<dyn Expr>>,
}

impl ScriptExpr {
    pub fn new(e: Option<Box<dyn Expr>>) -> Self {
        ScriptExpr {
            expr: e,
        }
    }

    pub fn run(&self, funcs: &FuncMap) -> ExprRes {
        match self.expr {
            Some(ref e) => {
                let mut state = Scope::new();

                e.eval(&mut state, funcs)
            },
            None    => Ok(Value::Null),
        }
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
                Some(func) => func.call(a, f, None),
                None => Err(Error::new(Type::RunTime(RunCode::FunctionNotFound))),
            }
        })
    }
}


// AST entry point for function
pub struct FuncRoot {
    arg_names: Vec<String>,
    stat_list: Vec<Box<dyn Statement>>,
}

impl FuncRoot {
    pub fn new(arg_names: Vec<String>, stat_list: Vec<Box<dyn Statement>>) -> Self {
        FuncRoot {
            arg_names: arg_names,
            stat_list: stat_list,
        }
    }

    pub fn call(&self, args: &[Value], f: &FuncMap, scope: Option<&[(String, Value)]>) -> ExprRes {
        let mut state = Scope::new();

        if args.len() != self.arg_names.len() {
            return mserr(Type::RunTime(RunCode::WrongNumberOfArguments));
        }

        if let Some(s) = scope {
            for (c,v) in s.iter() {
                state.new_var(&c, v.clone());
            }

            state.extend();
        }

        for (a,n) in args.iter().zip(self.arg_names.iter()) {
            state.new_var(&n, a.clone());
        }

        for s in &self.stat_list {
            match s.run(&mut state, f) {
                Signal::Done => {},
                Signal::Error(e) => return Err(e),
                Signal::Return(v) => return Ok(v),
                Signal::Continue => return mserr(Type::RunTime(RunCode::CannotContinue)),
                Signal::Break => return mserr(Type::RunTime(RunCode::CannotBreak)),
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

impl cmp::PartialEq for FuncRoot {
    fn eq(&self, _: &FuncRoot) -> bool {
        false
    }
}

impl fmt::Debug for FuncRoot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "closure")
    }
}
