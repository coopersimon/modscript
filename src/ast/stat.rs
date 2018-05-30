use super::{AstNode, Statement, Expr};
use runtime::{Value, Scope, Signal, FuncMap};

pub struct ScopeStat {
    code: Vec<Box<Statement>>,
}

pub struct VarDecl {
    name: String,
    assign: Option<Box<Expr>>,
}

pub struct AssignStat {
    name: String,
    assign: Box<Expr>,
}

pub struct IfStat {
    cond: Box<Expr>,
    then_stat: Box<Statement>,
    else_stat: Option<Box<Statement>>,
}

pub struct LoopStat {
    init: Box<Statement>,
    cond: Box<Expr>,
    end: Box<Statement>,
    loop_body: Box<Statement>,
}

pub struct WhileStat {
    cond: Box<Expr>,
    loop_body: Box<Statement>,
}

pub struct ReturnStat {
    expr: Option<Box<Expr>>,
}

// pub struct ContinueStat
// pub struct BreakStat


// IMPLS

impl ScopeStat {
    pub fn new(c: Vec<Box<Statement>>) -> Self {
        ScopeStat {
            /*code: match c{
                Some(c) => c,
                None => Vec::new(),
            },*/
            code: c,
        }
    }
}

impl AstNode for ScopeStat {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Statement for ScopeStat {
    fn run(&self, state: &mut Scope, f: &FuncMap) -> Signal {
        state.extend();

        for s in &self.code {
            match s.run(state, f) {
                Signal::Done => {},
                s => {state.reduce(); return s;},
            }
        }

        state.reduce();
        Signal::Done
    }
}


impl VarDecl {
    pub fn new(n: &str, a: Option<Box<Expr>>) -> Self {
        println!("var decl");
        VarDecl {
            name: n.to_string(),
            assign: a,
        }
    }
}

impl AstNode for VarDecl {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Statement for VarDecl {
    fn run(&self, state: &mut Scope, f: &FuncMap) -> Signal {
        let val = match self.assign {
            Some(ref e) => match e.eval(state, f) {
                Ok(v) => v,
                Err(e) => return Signal::Error(e),
            },
            None => Value::Null,
        };

        state.new_var(&self.name, val)
    }
}


impl AssignStat {
    pub fn new(n: &str, a: Box<Expr>) -> Self {
        AssignStat {
            name: n.to_string(),
            assign: a,
        }
    }
}

impl AstNode for AssignStat {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Statement for AssignStat {
    fn run(&self, state: &mut Scope, f: &FuncMap) -> Signal {
        let val = match self.assign.eval(state, f) {
            Ok(v) => v,
            Err(e) => return Signal::Error(e),
        };

        state.set_var(&self.name, val)
    }
}


impl IfStat {
    pub fn new(c: Box<Expr>, i: Box<Statement>, e: Option<Box<Statement>>) -> Self {
        IfStat {
            cond: c,
            then_stat: i,
            else_stat: e,
        }
    }
}

impl AstNode for IfStat {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Statement for IfStat {
    fn run(&self, state: &mut Scope, f: &FuncMap) -> Signal {
        let c = match self.cond.eval(state, f) {
            Ok(v) => v,
            Err(e) => return Signal::Error(e),
        };

        match c {
            Value::Bool(true) => return self.then_stat.run(state, f),
            Value::Bool(false) => {},
            Value::Int(i) => if i != 0 {return self.then_stat.run(state, f)},
            _ => return Signal::Error("Type error in if statement.".to_string()),
        }

        match self.else_stat {
            Some(ref s) => s.run(state, f),
            None => Signal::Done,
        }
    }
}


impl LoopStat {
    pub fn new(i: Box<Statement>, c: Box<Expr>, e: Box<Statement>, b: Box<Statement>) -> Self {
        LoopStat {
            init: i,
            cond: c,
            end: e,
            loop_body: b,
        }
    }
}

impl AstNode for LoopStat {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Statement for LoopStat {
    fn run(&self, state: &mut Scope, f: &FuncMap) -> Signal {
        match self.init.run(state, f) {
            Signal::Done => {},
            s => return s,
        }

        loop {
            match self.cond.eval(state, f) {
                Ok(v) => match v {
                    Value::Bool(true) => {},
                    Value::Bool(false) => break,
                    Value::Int(i) => if i == 0 {break},
                    _ => return Signal::Error("Type error in for loop condition.".to_string()),
                },
                Err(e) => return Signal::Error(e),
            }

            match self.loop_body.run(state, f) {
                Signal::Done => {},
                s => return s,
            }

            match self.end.run(state, f) {
                Signal::Done => {},
                s => return s,
            }
        }

        Signal::Done
    }
}


impl WhileStat {
    pub fn new(c: Box<Expr>, b: Box<Statement>) -> Self {
        WhileStat {
            cond: c,
            loop_body: b,
        }
    }
}

impl AstNode for WhileStat {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Statement for WhileStat {
    fn run(&self, state: &mut Scope, f: &FuncMap) -> Signal {
        loop {
            match self.cond.eval(state, f) {
                Ok(v) => match v {
                    Value::Bool(true) => {},
                    Value::Bool(false) => break,
                    Value::Int(i) => if i == 0 {break},
                    _ => return Signal::Error("Type error in while loop condition.".to_string()),
                },
                Err(e) => return Signal::Error(e),
            }

            match self.loop_body.run(state, f) {
                Signal::Done => {},
                s => return s,
            }
        }

        Signal::Done
    }
}


impl ReturnStat {
    pub fn new(e: Option<Box<Expr>>) -> Self {
        ReturnStat {
            expr: e,
        }
    }
}

impl AstNode for ReturnStat {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Statement for ReturnStat {
    fn run(&self, state: &mut Scope, f: &FuncMap) -> Signal {
        match self.expr {
            Some(ref e) => match e.eval(state, f) {
                Ok(v) => Signal::Return(v),
                Err(e) => Signal::Error(e),
            },
            None => Signal::Return(Value::Null),
        }
    }
}   
