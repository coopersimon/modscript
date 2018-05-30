use super::{Expr, AstNode};
use runtime::{Value, Scope, ExprRes, expr_err, FuncMap};

// DECLS
pub enum ValExpr {
    Var(String),
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
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

pub struct EqExpr {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub struct TrueEqExpr {
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
    fn eval(&self, state: &mut Scope, _: &FuncMap) -> ExprRes {
        //match *self {
        match self {
            &ValExpr::Var(ref n) => state.get_var(&n),
            &ValExpr::Int(ref v) => Ok(Value::Int(v.clone())),
            &ValExpr::Float(ref v) => Ok(Value::Float(v.clone())),
            &ValExpr::Text(ref v) => Ok(Value::Str(v.clone())),
            &ValExpr::Bool(ref v) => Ok(Value::Bool(v.clone())),
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

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => Ok(Value::Int(x + y)),
            (Value::Int(x),Value::Float(y)) => Ok(Value::Float(x as f64 + y)),
            (Value::Int(x),Value::Str(y)) => Ok(Value::Str(x.to_string() + &y)),
            (Value::Float(x),Value::Int(y)) => Ok(Value::Float(x + y as f64)),
            (Value::Float(x),Value::Float(y)) => Ok(Value::Float(x + y)),
            (Value::Float(x),Value::Str(y)) => Ok(Value::Str(x.to_string() + &y)),
            (Value::Str(x),Value::Int(y)) => Ok(Value::Str(x + &y.to_string())),
            (Value::Str(x),Value::Float(y)) => Ok(Value::Str(x + &y.to_string())),
            (Value::Str(x),Value::Str(y)) => Ok(Value::Str(x + &y)),
            (Value::Str(x),Value::Bool(true)) => Ok(Value::Str(x + "true")),
            (Value::Str(x),Value::Bool(false)) => Ok(Value::Str(x + "false")),
            (Value::Bool(true),Value::Str(y)) => Ok(Value::Str("true".to_string() + &y)),
            (Value::Bool(false),Value::Str(y)) => Ok(Value::Str("false".to_string() + &y)),
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => Ok(Value::Int(x - y)),
            (Value::Int(x),Value::Float(y)) => Ok(Value::Float(x as f64 - y)),
            (Value::Float(x),Value::Int(y)) => Ok(Value::Float(x - y as f64)),
            (Value::Float(x),Value::Float(y)) => Ok(Value::Float(x - y)),
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => Ok(Value::Int(x * y)),
            (Value::Int(x),Value::Float(y)) => Ok(Value::Float(x as f64 * y)),
            (Value::Float(x),Value::Int(y)) => Ok(Value::Float(x * y as f64)),
            (Value::Float(x),Value::Float(y)) => Ok(Value::Float(x * y)),
            (Value::Str(x),Value::Int(y)) => Ok(Value::Str(x.repeat(y as usize))),
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (_,Value::Int(0)) => expr_err("Divide by zero error."),
            (Value::Int(x),Value::Int(y)) => Ok(Value::Int(x / y)),
            (Value::Int(x),Value::Float(y)) => Ok(Value::Float(x as f64 / y)),
            (Value::Float(x),Value::Int(y)) => Ok(Value::Float(x / y as f64)),
            (Value::Float(x),Value::Float(y)) => Ok(Value::Float(x / y)),
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => Ok(Value::Int(x % y)),
            (_,_) => expr_err("Modulus type error."),
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => Ok(Value::Bool(x == y)),
            (Value::Int(x),Value::Float(y)) => Ok(Value::Bool(x == (y as i64))),
            (Value::Int(x),Value::Str(y)) => Ok(Value::Bool(x.to_string() == y)),
            (Value::Int(0),Value::Bool(true)) => Ok(Value::Bool(false)),
            (Value::Int(0),Value::Bool(false)) => Ok(Value::Bool(true)),
            (Value::Int(_),Value::Bool(true)) => Ok(Value::Bool(true)),
            (Value::Int(_),Value::Bool(false)) => Ok(Value::Bool(false)),
            (Value::Float(x),Value::Int(y)) => Ok(Value::Bool((x as i64) == y)),
            (Value::Float(x),Value::Float(y)) => Ok(Value::Bool(x == y)),
            (Value::Float(x),Value::Str(y)) => Ok(Value::Bool(x.to_string() == y)),
            (Value::Str(x),Value::Int(y)) => Ok(Value::Bool(x == y.to_string())),
            (Value::Str(x),Value::Float(y)) => Ok(Value::Bool(x == y.to_string())),
            (Value::Str(x),Value::Str(y)) => Ok(Value::Bool(x == y)),
            (Value::Str(x),Value::Bool(true)) => Ok(Value::Bool(x == "true")),
            (Value::Str(x),Value::Bool(false)) => Ok(Value::Bool(x == "false")),
            (Value::Bool(true),Value::Int(0)) => Ok(Value::Bool(false)),
            (Value::Bool(false),Value::Int(0)) => Ok(Value::Bool(true)),
            (Value::Bool(true),Value::Int(_)) => Ok(Value::Bool(true)),
            (Value::Bool(false),Value::Int(_)) => Ok(Value::Bool(false)),
            (Value::Bool(true),Value::Str(y)) => Ok(Value::Bool("true" == y)),
            (Value::Bool(false),Value::Str(y)) => Ok(Value::Bool("false" == y)),
            (Value::Bool(x),Value::Bool(y)) => Ok(Value::Bool(x == y)),
            (_,_) => Ok(Value::Bool(false)),
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => Ok(Value::Bool(x == y)),
            (Value::Float(x),Value::Float(y)) => Ok(Value::Bool(x == y)),
            (Value::Str(x),Value::Str(y)) => Ok(Value::Bool(x == y)),
            (Value::Bool(x),Value::Bool(y)) => Ok(Value::Bool(x == y)),
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => Ok(Value::Bool(x > y)),
            (Value::Int(x),Value::Float(y)) => Ok(Value::Bool((x as f64) > y)),
            (Value::Float(x),Value::Int(y)) => Ok(Value::Bool(x > (y as f64))),
            (Value::Float(x),Value::Float(y)) => Ok(Value::Bool(x > y)),
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => Ok(Value::Bool(x >= y)),
            (Value::Int(x),Value::Float(y)) => Ok(Value::Bool((x as f64) >= y)),
            (Value::Float(x),Value::Int(y)) => Ok(Value::Bool(x >= (y as f64))),
            (Value::Float(x),Value::Float(y)) => Ok(Value::Bool(x >= y)),
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => Ok(Value::Bool(x < y)),
            (Value::Int(x),Value::Float(y)) => Ok(Value::Bool((x as f64) < y)),
            (Value::Float(x),Value::Int(y)) => Ok(Value::Bool(x < (y as f64))),
            (Value::Float(x),Value::Float(y)) => Ok(Value::Bool(x < y)),
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => Ok(Value::Bool(x <= y)),
            (Value::Int(x),Value::Float(y)) => Ok(Value::Bool((x as f64) <= y)),
            (Value::Float(x),Value::Int(y)) => Ok(Value::Bool(x <= (y as f64))),
            (Value::Float(x),Value::Float(y)) => Ok(Value::Bool(x <= y)),
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
        let a = self.right.eval(state, f)?;

        match a {
            Value::Int(x) => Ok(Value::Int(!x)),
            Value::Bool(x) => Ok(Value::Bool(!x)),
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => Ok(Value::Int(x & y)),
            (Value::Bool(x),Value::Bool(y)) => Ok(Value::Bool(x && y)),
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => Ok(Value::Int(x | y)),
            (Value::Bool(x),Value::Bool(y)) => Ok(Value::Bool(x || y)),
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Value::Int(x),Value::Int(y)) => Ok(Value::Int(x ^ y)),
            (Value::Bool(x),Value::Bool(y)) => Ok(Value::Bool(if x == y {false} else {true})),
            (_,_) => expr_err("AND type error."),
        }
    }
}


impl FuncCall {
    pub fn new(p: &str, n: &str, a: Vec<Box<Expr>>) -> Self {
        FuncCall {
            package: p.to_string(),
            name: n.to_string(),
            /*args: match a {
                Some(v) => v,
                None => Vec::new(),
            },*/
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
