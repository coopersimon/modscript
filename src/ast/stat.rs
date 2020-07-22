use super::{AstNode, Statement, Expr, Assign};
use runtime::{Value, VType, Scope, Signal, FuncMap};
use error::{Error, Type, RunCode};

pub struct ScopeStat {
    code: Vec<Box<dyn Statement>>,
}

pub struct VarDecl {
    name: String,
    assign: Option<Box<dyn Expr>>,
}

pub struct AssignStat {
    name: String,
    assign: Box<dyn Expr>,
    child_op: Option<Box<dyn Assign>>,
}

pub struct ExprStat {
    expr: Box<dyn Expr>,
}

pub struct IfStat {
    cond: Box<dyn Expr>,
    then_stat: Box<dyn Statement>,
    else_stat: Option<Box<dyn Statement>>,
}

pub enum CaseType {
    Var(String),
    Value(Box<dyn Expr>),
}

pub struct MatchStat {
    cond: Box<dyn Expr>,
    cases: Vec<(CaseType, Box<dyn Statement>)>,
    otherwise: Option<Box<dyn Statement>>
}

pub struct WhileStat {
    cond: Box<dyn Expr>,
    loop_body: Box<dyn Statement>,
}

pub struct ForStat {
    e_name: String,
    list: Box<dyn Expr>,
    loop_body: Box<dyn Statement>,
}

pub struct ReturnStat {
    expr: Option<Box<dyn Expr>>,
}

pub struct ContinueStat {}

pub struct BreakStat {}


// IMPLS

impl ScopeStat {
    pub fn new(c: Vec<Box<dyn Statement>>) -> Self {
        ScopeStat {
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
    pub fn new(n: &str, a: Option<Box<dyn Expr>>) -> Self {
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
    pub fn new(n: &str, a: Box<dyn Expr>, c: Option<Box<dyn Assign>>) -> Self {
        AssignStat {
            name: n.to_string(),
            assign: a,
            child_op: c,
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
        let var = match state.get_var(&self.name) {
            Ok(v) => v,
            Err(e) => return Signal::Error(e),
        };

        let val = match self.assign.eval(state, f) {
            Ok(v) => v,
            Err(e) => return Signal::Error(e),
        };

        match self.child_op {
            Some(ref o) => o.assign(var, val, state, f),
            None    => state.set_var(&self.name, val),
        }
    }
}


impl ExprStat {
    pub fn new(e: Box<dyn Expr>) -> Self {
        ExprStat {
            expr: e,
        }
    }
}

impl AstNode for ExprStat {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Statement for ExprStat {
    fn run(&self, state: &mut Scope, f: &FuncMap) -> Signal {
        match self.expr.eval(state, f) {
            Ok(_) => Signal::Done,
            Err(e) => Signal::Error(e),
        }
    }
}


impl IfStat {
    pub fn new(c: Box<dyn Expr>, i: Box<dyn Statement>, e: Option<Box<dyn Statement>>) -> Self {
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
            Value::Val(VType::B(true)) => return self.then_stat.run(state, f),
            Value::Val(VType::B(false)) => {},
            Value::Val(VType::I(i)) => if i != 0 {return self.then_stat.run(state, f)},
            _ => return Signal::Error(Error::new(Type::RunTime(RunCode::TypeError))),
        }

        match self.else_stat {
            Some(ref s) => s.run(state, f),
            None => Signal::Done,
        }
    }
}


impl MatchStat {
    pub fn new(m: Box<dyn Expr>, c: Vec<(CaseType, Box<dyn Statement>)>, o: Option<Box<dyn Statement>>) -> Self {
        MatchStat {
            cond: m,
            cases: c,
            otherwise: o,
        }
    }
}

impl AstNode for MatchStat {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Statement for MatchStat {
    fn run(&self, state: &mut Scope, f: &FuncMap) -> Signal {
        let c = match self.cond.eval(state, f) {
            Ok(v) => v,
            Err(e) => return Signal::Error(e),
        };

        for (case, stat) in self.cases.iter() {
            match case {
                CaseType::Var(ref v)    => {
                    state.extend();
                    state.new_var(v, c.clone());
                    let ret = stat.run(state, f);
                    state.reduce();
                    return ret;
                }
                CaseType::Value(ref v)  => {
                    let val = match v.eval(state, f) {
                        Ok(v) => v,
                        Err(e) => return Signal::Error(e),
                    };

                    match c.equal_to(&val) {
                        Some(true) => return stat.run(state, f),
                        _ => (),
                    }
                }
            }
        }

        match self.otherwise {
            Some(ref s) => s.run(state, f),
            None => Signal::Done,
        }
    }
}


impl WhileStat {
    pub fn new(c: Box<dyn Expr>, b: Box<dyn Statement>) -> Self {
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
                    Value::Val(VType::B(true)) => {},
                    Value::Val(VType::B(false)) => break,
                    Value::Val(VType::I(i)) => if i == 0 {break},
                    _ => return Signal::Error(Error::new(Type::RunTime(RunCode::TypeError))),
                },
                Err(e) => return Signal::Error(e),
            }

            match self.loop_body.run(state, f) {
                Signal::Done => {},
                Signal::Continue => {},
                Signal::Break => break,
                s => return s,
            }
        }

        Signal::Done
    }
}


impl ForStat {
    pub fn new(e: String, l: Box<dyn Expr>, b: Box<dyn Statement>) -> Self {
        ForStat {
            e_name: e,
            list: l,
            loop_body: b,
        }
    }
}

impl AstNode for ForStat {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Statement for ForStat {
    fn run(&self, state: &mut Scope, f: &FuncMap) -> Signal {
        match self.list.eval(state, f) {
            Ok(v) => match v {
                Value::List(l) => {
                    state.extend();
                    state.new_var(&self.e_name, Value::Null);

                    for e in l.borrow().iter() {
                        state.set_var(&self.e_name, e.clone());

                        match self.loop_body.run(state, f) {
                            Signal::Done => {},
                            Signal::Continue => {},
                            Signal::Break => break,
                            s => {state.reduce(); return s},
                        }
                    }

                    state.reduce();

                    Signal::Done
                },
                /*Value::Str(s) => {
                    state.extend();
                    state.new_var(&self.e_name, Value::Null);

                    for e in l.borrow().iter() {
                        state.set_var(&self.e_name, e.clone());

                        match self.loop_body.run(state, f) {
                            Signal::Done => {},
                            Signal::Continue => {},
                            Signal::Break => break,
                            s => {state.reduce(); return s},
                        }
                    }

                    state.reduce();

                    Signal::Done
                },*/
                _ => return Signal::Error(Error::new(Type::RunTime(RunCode::TypeError))),
            },
            Err(e) => return Signal::Error(e),
        }

    }
}


impl ReturnStat {
    pub fn new(e: Option<Box<dyn Expr>>) -> Self {
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

impl ContinueStat {
    pub fn new() -> Self {
        ContinueStat {}
    }
}

impl AstNode for ContinueStat {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Statement for ContinueStat {
    fn run(&self, _: &mut Scope, _: &FuncMap) -> Signal {
        Signal::Continue
    }
}


impl BreakStat {
    pub fn new() -> Self {
        BreakStat {}
    }
}

impl AstNode for BreakStat {
    fn print(&self) -> String {
        "scope".to_string()
    }
}

impl Statement for BreakStat {
    fn run(&self, _: &mut Scope, _: &FuncMap) -> Signal {
        Signal::Break
    }
}
