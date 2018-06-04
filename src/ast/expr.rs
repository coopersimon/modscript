use super::{Expr, AstNode};
use runtime::{Value, Scope, ExprRes, expr_err, FuncMap};

use std::rc::Rc;
use std::cell::RefCell;

// DECLS
pub enum ValExpr {
    Var(String),
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
    List(Vec<Box<Expr>>),
}

pub struct IndexExpr {
    base: Box<Expr>,
    index: Box<Expr>,
}

pub struct AddExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct SubExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct MulExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct DivExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct ModExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct NegExpr {
    right: Box<Expr>,
}

pub struct EqExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct NEqExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct TrueEqExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct TrueNEqExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct GThanExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct GEqExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct LThanExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct LEqExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct NotExpr {
    right: Box<Expr>,
}

pub struct AndExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct OrExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct XorExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct FuncCall {
    package: String,
    name: String,
    args: Vec<Box<Expr>>,
}

/*pub struct IdChain {
    chain: Vec<String>,
    end_func: Option<FuncCall>,
}*/


// IMPLS

impl AstNode for ValExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for ValExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use std::rc::Rc;
        use std::cell::RefCell;
        //match *self {
        match self {
            &ValExpr::Var(ref n) => state.get_var(&n),
            &ValExpr::Int(ref v) => Ok(Value::Int(v.clone())),
            &ValExpr::Float(ref v) => Ok(Value::Float(v.clone())),
            &ValExpr::Text(ref v) => Ok(Value::Str(v.clone())),
            &ValExpr::Bool(ref v) => Ok(Value::Bool(v.clone())),
            &ValExpr::List(ref l) => {
                let r = Rc::new(RefCell::new(Vec::new()));
                for expr in l.iter() {
                    let el = expr.eval(state, f)?;
                    r.borrow_mut().push(el);
                }
                Ok(Value::List(r))
            },
        }
    }
}


impl IndexExpr {
    pub fn new(b: Box<Expr>, i: Box<Expr>) -> Self {
        IndexExpr {
            base: b,
            index: i,
        }
    }
}

impl AstNode for IndexExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for IndexExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let n = self.base.eval(state, f)?;
        let i = self.index.eval(state, f)?;

        match (n,i) {
            (List(l),Int(i)) => {
                let list = l.borrow();
                if (i >= 0) && ((i as usize) < list.len()) {
                    Ok(list[i as usize].clone())
                } else if (i < 0) && ((i.abs() as usize) <= list.len()) {
                    Ok(list[((list.len() as i64) + i) as usize].clone())
                } else {
                    expr_err("Index access out of bounds.")
                }
            },
            (List(_),_) => expr_err("Index access type error: can't index without int."),
            (a,Int(_)) => expr_err(&format!("Index access type error: can't index non-list object {}.", a)),
            (_,_) => expr_err("Index access type error."),
        }
    }
}


impl AddExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        AddExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for AddExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for AddExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        use Value::*;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Int(x + y)),
            (Int(x),Float(y)) => Ok(Float(x as f64 + y)),
            (Int(x),Str(y)) => Ok(Str(x.to_string() + &y)),
            (Float(x),Int(y)) => Ok(Float(x + y as f64)),
            (Float(x),Float(y)) => Ok(Float(x + y)),
            (Float(x),Str(y)) => Ok(Str(x.to_string() + &y)),
            (Str(x),Int(y)) => Ok(Str(x + &y.to_string())),
            (Str(x),Float(y)) => Ok(Str(x + &y.to_string())),
            (Str(x),Str(y)) => Ok(Str(x + &y)),
            (Str(x),Bool(true)) => Ok(Str(x + "true")),
            (Str(x),Bool(false)) => Ok(Str(x + "false")),
            (Bool(true),Str(y)) => Ok(Str("true".to_string() + &y)),
            (Bool(false),Str(y)) => Ok(Str("false".to_string() + &y)),
            (List(x),List(y)) => {
                let x = x.borrow();
                let y = y.borrow();
                let list = Rc::new(RefCell::new([&x[..], &y[..]].concat()));
                Ok(List(list))
            },
            (_,_) => expr_err("Addition type error."),
        }
    }
}


impl SubExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        SubExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for SubExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for SubExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Int(x - y)),
            (Int(x),Float(y)) => Ok(Float(x as f64 - y)),
            (Float(x),Int(y)) => Ok(Float(x - y as f64)),
            (Float(x),Float(y)) => Ok(Float(x - y)),
            (_,_) => expr_err("Subtraction type error."),
        }
    }
}


impl MulExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        MulExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for MulExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for MulExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Int(x * y)),
            (Int(x),Float(y)) => Ok(Float(x as f64 * y)),
            (Float(x),Int(y)) => Ok(Float(x * y as f64)),
            (Float(x),Float(y)) => Ok(Float(x * y)),
            (Str(x),Int(y)) => Ok(Str(x.repeat(y as usize))),
            (List(x),Int(y)) => {
                if y < 0 {
                    expr_err("Can't duplicate list by negative value.")
                } else {
                    let x = x.borrow();
                    let list = Rc::new(RefCell::new(Vec::new()));
                    for _ in 0..y {
                        list.borrow_mut().extend_from_slice(&x);
                    }
                    Ok(List(list))
                }
            },
            (_,_) => expr_err("Multiplication type error."),
        }
    }
}


impl DivExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        DivExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for DivExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for DivExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (_,Int(0)) => expr_err("Divide by zero error."),
            (Int(x),Int(y)) => Ok(Int(x / y)),
            (Int(x),Float(y)) => Ok(Float(x as f64 / y)),
            (Float(x),Int(y)) => Ok(Float(x / y as f64)),
            (Float(x),Float(y)) => Ok(Float(x / y)),
            (_,_) => expr_err("Division type error."),
        }
    }
}


impl ModExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        ModExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for ModExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for ModExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Int(x % y)),
            (_,_) => expr_err("Modulus type error."),
        }
    }
}


impl NegExpr {
    pub fn new(r: Box<Expr>) -> Self {
        NegExpr {
            right: r,
        }
    }
}

impl AstNode for NegExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for NegExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.right.eval(state, f)?;

        match a {
            Int(x) => Ok(Int(-x)),
            Float(x) => Ok(Float(-x)),
            _ => expr_err("Negation type error."),
        }
    }
}


impl EqExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        EqExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for EqExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for EqExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Bool(x == y)),
            (Int(x),Float(y)) => Ok(Bool(x == (y as i64))),
            (Int(x),Str(y)) => Ok(Bool(x.to_string() == y)),
            (Int(0),Bool(true)) => Ok(Bool(false)),
            (Int(0),Bool(false)) => Ok(Bool(true)),
            (Int(_),Bool(true)) => Ok(Bool(true)),
            (Int(_),Bool(false)) => Ok(Bool(false)),
            (Float(x),Int(y)) => Ok(Bool((x as i64) == y)),
            (Float(x),Float(y)) => Ok(Bool(x == y)),
            (Float(x),Str(y)) => Ok(Bool(x.to_string() == y)),
            (Str(x),Int(y)) => Ok(Bool(x == y.to_string())),
            (Str(x),Float(y)) => Ok(Bool(x == y.to_string())),
            (Str(x),Str(y)) => Ok(Bool(x == y)),
            (Str(x),Bool(true)) => Ok(Bool(x == "true")),
            (Str(x),Bool(false)) => Ok(Bool(x == "false")),
            (Bool(true),Int(0)) => Ok(Bool(false)),
            (Bool(false),Int(0)) => Ok(Bool(true)),
            (Bool(true),Int(_)) => Ok(Bool(true)),
            (Bool(false),Int(_)) => Ok(Bool(false)),
            (Bool(true),Str(y)) => Ok(Bool("true" == y)),
            (Bool(false),Str(y)) => Ok(Bool("false" == y)),
            (Bool(x),Bool(y)) => Ok(Bool(x == y)),
            (Null,Null) => Ok(Bool(true)),
            (_,_) => Ok(Bool(false)),
            //(_,_) => expr_err("Equality check type error.".to_string()),
        }
    }
}


impl NEqExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        NEqExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for NEqExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for NEqExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Bool(x != y)),
            (Int(x),Float(y)) => Ok(Bool(x != (y as i64))),
            (Int(x),Str(y)) => Ok(Bool(x.to_string() != y)),
            (Int(0),Bool(true)) => Ok(Bool(true)),
            (Int(0),Bool(false)) => Ok(Bool(false)),
            (Int(_),Bool(true)) => Ok(Bool(false)),
            (Int(_),Bool(false)) => Ok(Bool(true)),
            (Float(x),Int(y)) => Ok(Bool((x as i64) != y)),
            (Float(x),Float(y)) => Ok(Bool(x != y)),
            (Float(x),Str(y)) => Ok(Bool(x.to_string() != y)),
            (Str(x),Int(y)) => Ok(Bool(x != y.to_string())),
            (Str(x),Float(y)) => Ok(Bool(x != y.to_string())),
            (Str(x),Str(y)) => Ok(Bool(x != y)),
            (Str(x),Bool(true)) => Ok(Bool(x != "true")),
            (Str(x),Bool(false)) => Ok(Bool(x != "false")),
            (Bool(true),Int(0)) => Ok(Bool(true)),
            (Bool(false),Int(0)) => Ok(Bool(false)),
            (Bool(true),Int(_)) => Ok(Bool(false)),
            (Bool(false),Int(_)) => Ok(Bool(true)),
            (Bool(true),Str(y)) => Ok(Bool("true" != y)),
            (Bool(false),Str(y)) => Ok(Bool("false" != y)),
            (Bool(x),Bool(y)) => Ok(Bool(x != y)),
            (Null,Null) => Ok(Bool(false)),
            (_,_) => Ok(Bool(true)),
            //(_,_) => expr_err("Equality check type error.".to_string()),
        }
    }
}


impl TrueEqExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        TrueEqExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for TrueEqExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for TrueEqExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Bool(x == y)),
            (Float(x),Float(y)) => Ok(Bool(x == y)),
            (Str(x),Str(y)) => Ok(Bool(x == y)),
            (Bool(x),Bool(y)) => Ok(Bool(x == y)),
            (_,_) => expr_err("Equality check type error."),
        }
    }
}


impl TrueNEqExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        TrueNEqExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for TrueNEqExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for TrueNEqExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Bool(x != y)),
            (Float(x),Float(y)) => Ok(Bool(x != y)),
            (Str(x),Str(y)) => Ok(Bool(x != y)),
            (Bool(x),Bool(y)) => Ok(Bool(x != y)),
            (_,_) => expr_err("Equality check type error."),
        }
    }
}


impl GThanExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        GThanExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for GThanExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for GThanExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Bool(x > y)),
            (Int(x),Float(y)) => Ok(Bool((x as f64) > y)),
            (Float(x),Int(y)) => Ok(Bool(x > (y as f64))),
            (Float(x),Float(y)) => Ok(Bool(x > y)),
            (_,_) => expr_err("Greater than type error."),
        }
    }
}


impl GEqExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        GEqExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for GEqExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for GEqExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Bool(x >= y)),
            (Int(x),Float(y)) => Ok(Bool((x as f64) >= y)),
            (Float(x),Int(y)) => Ok(Bool(x >= (y as f64))),
            (Float(x),Float(y)) => Ok(Bool(x >= y)),
            (_,_) => expr_err("Greater than or equal to type error."),
        }
    }
}


impl LThanExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        LThanExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for LThanExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for LThanExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Bool(x < y)),
            (Int(x),Float(y)) => Ok(Bool((x as f64) < y)),
            (Float(x),Int(y)) => Ok(Bool(x < (y as f64))),
            (Float(x),Float(y)) => Ok(Bool(x < y)),
            (_,_) => expr_err("Less than type error."),
        }
    }
}


impl LEqExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        LEqExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for LEqExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for LEqExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Bool(x <= y)),
            (Int(x),Float(y)) => Ok(Bool((x as f64) <= y)),
            (Float(x),Int(y)) => Ok(Bool(x <= (y as f64))),
            (Float(x),Float(y)) => Ok(Bool(x <= y)),
            (_,_) => expr_err("Less than or equal to type error."),
        }
    }
}


impl NotExpr {
    pub fn new(e: Box<Expr>) -> Self {
        NotExpr {
            right: e,
        }
    }
}

impl AstNode for NotExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for NotExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.right.eval(state, f)?;

        match a {
            Int(x) => Ok(Int(!x)),
            Bool(x) => Ok(Bool(!x)),
            _ => expr_err("Not type error."),
        }
    }
}


impl AndExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        AndExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for AndExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for AndExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Int(x & y)),
            (Bool(x),Bool(y)) => Ok(Bool(x && y)),
            (_,_) => expr_err("AND type error."),
        }
    }
}


impl OrExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        OrExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for OrExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for OrExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Int(x | y)),
            (Bool(x),Bool(y)) => Ok(Bool(x || y)),
            (_,_) => expr_err("OR type error."),
        }
    }
}


impl XorExpr {
    pub fn new(l: Box<Expr>, r: Box<Expr>) -> Self {
        XorExpr {
            left: l,
            right: r,
        }
    }
}

impl AstNode for XorExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for XorExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Int(x),Int(y)) => Ok(Int(x ^ y)),
            (Bool(x),Bool(y)) => Ok(Bool(if x == y {false} else {true})),
            (_,_) => expr_err("AND type error."),
        }
    }
}


impl FuncCall {
    pub fn new(p: &str, n: &str, a: Vec<Box<Expr>>) -> Self {
        FuncCall {
            package: p.to_string(),
            name: n.to_string(),
            args: a,
        }
    }
}

impl AstNode for FuncCall {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for FuncCall {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        let mut func_args = Vec::new();

        for a in &self.args {
            match a.eval(state, f) {
                Ok(v) => func_args.push(v),
                e => return e,
            }
        }

        f.call_fn(&self.package, &self.name, &func_args)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use runtime::{Scope, Value};

    // ADD

    #[test]
    fn add_int_consts() {
        let mut state = Scope::new();
        let f = FuncMap::new();

        let add = AddExpr::new(Box::new(ValExpr::Int(25)), Box::new(ValExpr::Int(12)));

        assert_eq!(add.eval(&mut state, &f), Ok(Value::Int(37)));
    }

    #[test]
    fn add_int_to_float() {
        let mut state = Scope::new();
        let f = FuncMap::new();

        let add = AddExpr::new(Box::new(ValExpr::Int(25)), Box::new(ValExpr::Float(3.3)));

        assert_eq!(add.eval(&mut state, &f), Ok(Value::Float(28.3)));
    }

    #[test]
    fn add_int_to_text() {
        let mut state = Scope::new();
        let f = FuncMap::new();

        let add = AddExpr::new(Box::new(ValExpr::Int(25)), Box::new(ValExpr::Text(" twenty five".to_string())));

        assert_eq!(add.eval(&mut state, &f), Ok(Value::Str("25 twenty five".to_string())));
    }

    #[test]
    fn add_text_to_float() {
        let mut state = Scope::new();
        let f = FuncMap::new();

        let add = AddExpr::new(Box::new(ValExpr::Text("x = ".to_string())), Box::new(ValExpr::Float(3.3)));

        assert_eq!(add.eval(&mut state, &f), Ok(Value::Str("x = 3.3".to_string())));
    }
}
