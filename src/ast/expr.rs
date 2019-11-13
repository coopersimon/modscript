use super::{Expr, AstNode, FuncRoot};
use runtime::{Value, VType, Scope, ExprRes, FuncMap, core_func_call, hash_value, equal};
use error::{mserr, Type, RunCode, Error};

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

macro_rules! refstr {
    ($s:expr) => {
        Value::Str(Rc::new(RefCell::new($s)))
    };
}

// DECLS
pub enum ValExpr {
    //Id(String),
    QualId(String, String),
    Ref(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Pair(Box<dyn Expr>, Box<dyn Expr>),
    Text(String),
    List(Vec<Box<dyn Expr>>),
    Obj(Vec<(String,Box<dyn Expr>)>),
    Map(Vec<(Box<dyn Expr>,Box<dyn Expr>)>),
    Closure(Rc<RefCell<FuncRoot>>),
    Null,
}

pub struct RangeExpr {
    start: Box<dyn Expr>,
    step: Option<Box<dyn Expr>>,
    end: Box<dyn Expr>,
}

pub struct IndexExpr {
    base: Box<dyn Expr>,
    index: Box<dyn Expr>,
}

pub struct AccessExpr {
    base: Box<dyn Expr>,
    access_id: String,
}

pub struct AddExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct SubExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct MulExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct DivExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct ModExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct NegExpr {
    right: Box<dyn Expr>,
}

pub struct EqExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct NEqExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct TrueEqExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct TrueNEqExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct GThanExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct GEqExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct LThanExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct LEqExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct NotExpr {
    right: Box<dyn Expr>,
}

pub struct AndExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct OrExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct XorExpr {
    left: Box<dyn Expr>,
    right: Box<dyn Expr>,
}

pub struct FuncCall {
    base: Box<dyn Expr>,
    args: Vec<Box<dyn Expr>>,
}

pub struct CoreFuncCall {
    name: String,
    base: Box<dyn Expr>,
    args: Vec<Box<dyn Expr>>,
}


// IMPLS

impl AstNode for ValExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for ValExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use self::VType::*;
        use self::ValExpr::*;
        match *self {
            // TODO: function calls to unqualified Id
            //&ValExpr::Id(ref n) => state.get_var(&n),
            QualId(ref p, ref n) => match state.get_var(&n) {
                Ok(v) => Ok(v),
                Err(_e) => {
                    let rp = Rc::new(RefCell::new(p.clone()));
                    let rn = Rc::new(RefCell::new(n.clone()));
                    Ok(Value::Func(rp, rn))
                },
            },
            Ref(ref n) => state.get_ref(&n),
            Int(ref v) => Ok(Value::Val(I(v.clone()))),
            Float(ref v) => Ok(Value::Val(F(v.clone()))),
            Bool(ref v) => Ok(Value::Val(B(v.clone()))),
            Pair(ref l, ref r) => {
                let l = l.eval(state, f)?;
                let r = r.eval(state, f)?;
                let rl = Rc::new(RefCell::new(l));
                let rr = Rc::new(RefCell::new(r));
                Ok(Value::Pair(rl,rr))
            },
            Text(ref v) => {
                let r = Rc::new(RefCell::new(v.clone()));
                Ok(Value::Str(r))
            },
            List(ref l) => {
                let r = Rc::new(RefCell::new(Vec::new()));
                for expr in l.iter() {
                    let el = expr.eval(state, f)?;
                    r.borrow_mut().push(el);
                }
                Ok(Value::List(r))
            },
            Obj(ref o) => {
                let r = Rc::new(RefCell::new(HashMap::new()));
                for &(ref n, ref expr) in o.iter() {
                    let el = expr.eval(state, f)?;
                    r.borrow_mut().insert(n.clone(), el);
                }
                Ok(Value::Obj(r))
            },
            Map(ref m) => {
                let r = Rc::new(RefCell::new(HashMap::new()));
                for &(ref k, ref v) in m.iter() {
                    let kval = k.eval(state, f)?;
                    let keyhash = hash_value(&kval)?;
                    let vval = v.eval(state, f)?;
                    r.borrow_mut().insert(keyhash, (kval, vval));
                }
                Ok(Value::Map(r))
            },
            Closure(ref c) => {
                let r = Rc::new(RefCell::new(state.get_scope_refs()));
                Ok(Value::Closure(c.clone(), r))
            },
            Null => Ok(Value::Null),
        }
    }
}


impl RangeExpr {
    pub fn new(s: Box<dyn Expr>, step: Option<Box<dyn Expr>>, e: Box<dyn Expr>) -> Self {
        RangeExpr {
            start: s,
            step: step,
            end: e,
        }
    }
}

impl AstNode for RangeExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for RangeExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        use self::VType::*;

        let start = self.start.eval(state, f)?;
        let end = self.end.eval(state, f)?;
        let step = self.step.as_ref().map_or(Ok(1), |s| match s.eval(state, f)? {
            Val(I(i)) => Ok(i),
            Ref(ref r) => match *r.borrow() {
                I(i) => Ok(i),
                _ => Err(Error::new(Type::RunTime(RunCode::TypeError))),
            },
            _ => Err(Error::new(Type::RunTime(RunCode::TypeError))),
        })?;

        let (mut start_num, end_num) = match (start,end) {
            (Val(I(s)), Val(I(e))) => (s,e),
            _ => return mserr(Type::RunTime(RunCode::TypeError)),
        };

        if start_num >= end_num {
            return mserr(Type::RunTime(RunCode::InvalidRange));
        }

        let r = Rc::new(RefCell::new(Vec::new()));

        while start_num < end_num {
            r.borrow_mut().push(Val(I(start_num)));
            start_num += step;
        }

        Ok(Value::List(r))
    }
}


impl IndexExpr {
    pub fn new(b: Box<dyn Expr>, i: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let l = self.base.eval(state, f)?;
        let i = self.index.eval(state, f)?;

        match (l,i) {
            (List(l),Val(I(i))) => {
                let list = l.borrow();

                let index = if (i >= 0) && ((i as usize) < list.len()) {
                    i as usize
                } else if (i < 0) && ((i.abs() as usize) <= list.len()) {
                    ((list.len() as i64) + i) as usize
                } else {
                    return mserr(Type::RunTime(RunCode::OutOfBounds));
                };

                Ok(list[index].clone())
            },
            (List(_),_) => mserr(Type::RunTime(RunCode::TypeError)),
            /*(Str(s),Int(i)) => {
                let text = s.borrow();
                /*if (i >= 0) && ((i as usize) < text.len()) {
                    Ok(list[i as usize].clone())
                } else if (i < 0) && ((i.abs() as usize) <= list.len()) {
                    Ok(list[((list.len() as i64) + i) as usize].clone())
                } else {
                    expr_err("Index access out of bounds.")
                }*/
            },*/
            (Map(m),iv) => {
                let map = m.borrow();
                let index = hash_value(&iv)?;
                let (_key, val) = match map.get(&index) {
                    Some(v) => v,
                    None    => return mserr(Type::RunTime(RunCode::OutOfBounds)),
                };
                Ok(val.clone())
            },
            _ => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl AccessExpr {
    pub fn new(b: Box<dyn Expr>, a: &str) -> Self {
        AccessExpr {
            base: b,
            access_id: a.to_string(),
        }
    }
}

impl AstNode for AccessExpr {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for AccessExpr {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        use Value::*;
        let o = self.base.eval(state, f)?;

        match o {
            Obj(o) => {
                let obj = o.borrow();
                match obj.get(&self.access_id) {
                    Some(v) => Ok(v.clone()),
                    None => mserr(Type::RunTime(RunCode::FieldNotFound)),
                }
            },
            _ => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl AddExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use Value::*;
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Val(I(x)),Val(I(y))) => Ok(Val(I(x + y))),
            (Val(I(x)),Val(F(y))) => Ok(Val(F(x as f64 + y))),
            (Val(I(x)),Str(y)) => Ok(refstr!(x.to_string() + &*y.borrow())),
            (Val(F(x)),Val(I(y))) => Ok(Val(F(x + y as f64))),
            (Val(F(x)),Val(F(y))) => Ok(Val(F(x + y))),
            (Val(F(x)),Str(y)) => Ok(refstr!(x.to_string() + &*y.borrow())),
            (Str(x),Val(I(y))) => Ok(refstr!(x.borrow().clone() + &y.to_string())),
            (Str(x),Val(F(y))) => Ok(refstr!(x.borrow().clone() + &y.to_string())),
            (Str(x),Str(y)) => Ok(refstr!(x.borrow().clone() + &*y.borrow())),
            (Str(x),Val(B(true))) => Ok(refstr!(x.borrow().clone() + "true")),
            (Str(x),Val(B(false))) => Ok(refstr!(x.borrow().clone() + "false")),
            (Val(B(true)),Str(y)) => Ok(refstr!("true".to_string() + &*y.borrow())),
            (Val(B(false)),Str(y)) => Ok(refstr!("false".to_string() + &*y.borrow())),
            (List(x),List(y)) => {
                let x = x.borrow();
                let y = y.borrow();
                let list = Rc::new(RefCell::new([&x[..], &y[..]].concat()));
                Ok(List(list))
            },
            (_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl SubExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Val(I(x)),Val(I(y))) => Ok(Val(I(x - y))),
            (Val(I(x)),Val(F(y))) => Ok(Val(F(x as f64 - y))),
            (Val(F(x)),Val(I(y))) => Ok(Val(F(x - y as f64))),
            (Val(F(x)),Val(F(y))) => Ok(Val(F(x - y))),
            (_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl MulExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Val(I(x)),Val(I(y))) => Ok(Val(I(x * y))),
            (Val(I(x)),Val(F(y))) => Ok(Val(F(x as f64 * y))),
            (Val(F(x)),Val(I(y))) => Ok(Val(F(x * y as f64))),
            (Val(F(x)),Val(F(y))) => Ok(Val(F(x * y))),
            (Str(x),Val(I(y))) => Ok(refstr!(x.borrow().repeat(y as usize))),
            (List(x),Val(I(y))) => {
                if y < 0 {
                    mserr(Type::RunTime(RunCode::InvalidNegative)) // Negative value?
                } else {
                    let x = x.borrow();
                    let list = Rc::new(RefCell::new(Vec::new()));
                    for _ in 0..y {
                        list.borrow_mut().extend_from_slice(&x);
                    }
                    Ok(List(list))
                }
            },
            (_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl DivExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (_,Val(I(0))) => mserr(Type::RunTime(RunCode::DivideByZero)),
            //(_,Val(F(0.0))) => mserr(Type::RunTime(RunCode::DivideByZero)), TODO: sort this
            (Val(I(x)),Val(I(y))) => Ok(Val(I(x / y))),
            (Val(I(x)),Val(F(y))) => Ok(Val(F(x as f64 / y))),
            (Val(F(x)),Val(I(y))) => Ok(Val(F(x / y as f64))),
            (Val(F(x)),Val(F(y))) => Ok(Val(F(x / y))),
            (_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl ModExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Val(I(x)),Val(I(y))) => Ok(Val(I(x % y))),
            (_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl NegExpr {
    pub fn new(r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.right.eval(state, f)?;

        match a {
            Val(I(x)) => Ok(Val(I(-x))),
            Val(F(x)) => Ok(Val(F(-x))),
            _ => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl EqExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Val(I(x)),Val(I(y))) => Ok(Val(B(x == y))),
            (Val(I(x)),Val(F(y))) => Ok(Val(B(x == (y as i64)))),
            (Val(I(x)),Str(y)) => Ok(Val(B(x.to_string() == *y.borrow()))),
            (Val(I(0)),Val(B(true))) => Ok(Val(B(false))),
            (Val(I(0)),Val(B(false))) => Ok(Val(B(true))),
            (Val(I(_)),Val(B(true))) => Ok(Val(B(true))),
            (Val(I(_)),Val(B(false))) => Ok(Val(B(false))),
            (Val(F(x)),Val(I(y))) => Ok(Val(B((x as i64) == y))),
            (Val(F(x)),Val(F(y))) => Ok(Val(B(x == y))),
            (Val(F(x)),Str(y)) => Ok(Val(B(x.to_string() == *y.borrow()))),
            (Str(x),Val(I(y))) => Ok(Val(B(*x.borrow() == y.to_string()))),
            (Str(x),Val(F(y))) => Ok(Val(B(*x.borrow() == y.to_string()))),
            (Str(x),Str(y)) => Ok(Val(B(*x.borrow() == *y.borrow()))),
            (Str(x),Val(B(true))) => Ok(Val(B(*x.borrow() == "true"))),
            (Str(x),Val(B(false))) => Ok(Val(B(*x.borrow() == "false"))),
            (Val(B(true)),Val(I(0))) => Ok(Val(B(false))),
            (Val(B(false)),Val(I(0))) => Ok(Val(B(true))),
            (Val(B(true)),Val(I(_))) => Ok(Val(B(true))),
            (Val(B(false)),Val(I(_))) => Ok(Val(B(false))),
            (Val(B(true)),Str(y)) => Ok(Val(B("true" == *y.borrow()))),
            (Val(B(false)),Str(y)) => Ok(Val(B("false" == *y.borrow()))),
            (Val(B(x)),Val(B(y))) => Ok(Val(B(x == y))),
            (List(x),List(y)) => {
                if x.borrow().len() != y.borrow().len() {
                    return Ok(Val(B(false)));
                }
                for (i,j) in x.borrow().iter().zip(y.borrow().iter()) {
                    if i != j {
                        return Ok(Val(B(false)));
                    }
                }
                Ok(Val(B(true)))
            },
            (Obj(x),Obj(y)) => {
                if x.borrow().len() != y.borrow().len() {
                    return Ok(Val(B(false)));
                }
                for ((fa,va),(fb,vb)) in x.borrow().iter().zip(y.borrow().iter()) {
                    if (fa != fb) || (va != vb) {
                        return Ok(Val(B(false)));
                    }
                }
                Ok(Val(B(true)))
            },
            (Null,Null) => Ok(Val(B(true))),
            (_,_) => Ok(Val(B(false))),
            //(_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl NEqExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Val(I(x)),Val(I(y))) => Ok(Val(B(x != y))),
            (Val(I(x)),Val(F(y))) => Ok(Val(B(x != (y as i64)))),
            (Val(I(x)),Str(y)) => Ok(Val(B(x.to_string() != *y.borrow()))),
            (Val(I(0)),Val(B(true))) => Ok(Val(B(true))),
            (Val(I(0)),Val(B(false))) => Ok(Val(B(false))),
            (Val(I(_)),Val(B(true))) => Ok(Val(B(false))),
            (Val(I(_)),Val(B(false))) => Ok(Val(B(true))),
            (Val(F(x)),Val(I(y))) => Ok(Val(B((x as i64) != y))),
            (Val(F(x)),Val(F(y))) => Ok(Val(B(x != y))),
            (Val(F(x)),Str(y)) => Ok(Val(B(x.to_string() != *y.borrow()))),
            (Str(x),Val(I(y))) => Ok(Val(B(*x.borrow() != y.to_string()))),
            (Str(x),Val(F(y))) => Ok(Val(B(*x.borrow() != y.to_string()))),
            (Str(x),Str(y)) => Ok(Val(B(*x.borrow() != *y.borrow()))),
            (Str(x),Val(B(true))) => Ok(Val(B(*x.borrow() != "true"))),
            (Str(x),Val(B(false))) => Ok(Val(B(*x.borrow() != "false"))),
            (Val(B(true)),Val(I(0))) => Ok(Val(B(true))),
            (Val(B(false)),Val(I(0))) => Ok(Val(B(false))),
            (Val(B(true)),Val(I(_))) => Ok(Val(B(false))),
            (Val(B(false)),Val(I(_))) => Ok(Val(B(true))),
            (Val(B(true)),Str(y)) => Ok(Val(B("true" != *y.borrow()))),
            (Val(B(false)),Str(y)) => Ok(Val(B("false" != *y.borrow()))),
            (Val(B(x)),Val(B(y))) => Ok(Val(B(x != y))),
            (List(x),List(y)) => {
                if x.borrow().len() != y.borrow().len() {
                    return Ok(Val(B(true)));
                }
                for (i,j) in x.borrow().iter().zip(y.borrow().iter()) {
                    if i != j {
                        return Ok(Val(B(true)));
                    }
                }
                Ok(Val(B(false)))
            },
            (Obj(x),Obj(y)) => {
                if x.borrow().len() != y.borrow().len() {
                    return Ok(Val(B(true)));
                }
                for ((fa,va),(fb,vb)) in x.borrow().iter().zip(y.borrow().iter()) {
                    if (fa != fb) || (va != vb) {
                        return Ok(Val(B(true)));
                    }
                }
                Ok(Val(B(false)))
            },
            (Null,Null) => Ok(Val(B(false))),
            (_,_) => Ok(Val(B(true))),
            //(_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl TrueEqExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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

        match equal(&a, &b) {
            Some(res) => Ok(Value::Val(VType::B(res))),
            None      => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl TrueNEqExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match equal(&a, &b) {
            Some(res) => Ok(Value::Val(VType::B(!res))),
            None      => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl GThanExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Val(I(x)),Val(I(y))) => Ok(Val(B(x > y))),
            (Val(I(x)),Val(F(y))) => Ok(Val(B((x as f64) > y))),
            (Val(F(x)),Val(I(y))) => Ok(Val(B(x > (y as f64)))),
            (Val(F(x)),Val(F(y))) => Ok(Val(B(x > y))),
            (_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl GEqExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Val(I(x)),Val(I(y))) => Ok(Val(B(x >= y))),
            (Val(I(x)),Val(F(y))) => Ok(Val(B((x as f64) >= y))),
            (Val(F(x)),Val(I(y))) => Ok(Val(B(x >= (y as f64)))),
            (Val(F(x)),Val(F(y))) => Ok(Val(B(x >= y))),
            (_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl LThanExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Val(I(x)),Val(I(y))) => Ok(Val(B(x < y))),
            (Val(I(x)),Val(F(y))) => Ok(Val(B((x as f64) < y))),
            (Val(F(x)),Val(I(y))) => Ok(Val(B(x < (y as f64)))),
            (Val(F(x)),Val(F(y))) => Ok(Val(B(x < y))),
            (_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl LEqExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Val(I(x)),Val(I(y))) => Ok(Val(B(x <= y))),
            (Val(I(x)),Val(F(y))) => Ok(Val(B((x as f64) <= y))),
            (Val(F(x)),Val(I(y))) => Ok(Val(B(x <= (y as f64)))),
            (Val(F(x)),Val(F(y))) => Ok(Val(B(x <= y))),
            (_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl NotExpr {
    pub fn new(e: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.right.eval(state, f)?;

        match a {
            Val(I(x)) => Ok(Val(I(!x))),
            Val(B(x)) => Ok(Val(B(!x))),
            _ => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl AndExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Val(I(x)),Val(I(y))) => Ok(Val(I(x & y))),
            (Val(B(x)),Val(B(y))) => Ok(Val(B(x && y))),
            (_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl OrExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Val(I(x)),Val(I(y))) => Ok(Val(I(x | y))),
            (Val(B(x)),Val(B(y))) => Ok(Val(B(x || y))),
            (_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl XorExpr {
    pub fn new(l: Box<dyn Expr>, r: Box<dyn Expr>) -> Self {
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
        use self::VType::*;
        let a = self.left.eval(state, f)?;
        let b = self.right.eval(state, f)?;

        match (a,b) {
            (Val(I(x)),Val(I(y))) => Ok(Val(I(x ^ y))),
            (Val(B(x)),Val(B(y))) => Ok(Val(B(if x == y {false} else {true}))),
            (_,_) => mserr(Type::RunTime(RunCode::TypeError)),
        }
    }
}


impl FuncCall {
    pub fn new(b: Box<dyn Expr>, a: Vec<Box<dyn Expr>>) -> Self {
        FuncCall {
            base: b,
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
        let base = self.base.eval(state, f)?;

        let mut func_args = Vec::new();

        for a in &self.args {
            match a.eval(state, f) {
                Ok(v) => func_args.push(v),
                e => return e,
            }
        }

        match base {
            Value::Func(package, name) => f.call_fn(&package.borrow(), &name.borrow(), &func_args),
            Value::Closure(func, captures) => func.borrow().call(&func_args, f, Some(&captures.borrow())),
            _ => mserr(Type::RunTime(RunCode::InvalidCall)),
        }
    }
}


impl CoreFuncCall {
    pub fn new(n: &str, b: Box<dyn Expr>, a: Vec<Box<dyn Expr>>) -> Self {
        CoreFuncCall {
            name: n.to_string(),
            base: b,
            args: a,
        }
    }
}

impl AstNode for CoreFuncCall {
    fn print(&self) -> String {
        "Val".to_string()
    }
}

impl Expr for CoreFuncCall {
    fn eval(&self, state: &mut Scope, f: &FuncMap) -> ExprRes {
        let base = self.base.eval(state, f)?;

        let mut func_args = Vec::new();

        for a in &self.args {
            match a.eval(state, f) {
                Ok(v) => func_args.push(v),
                e => return e,
            }
        }

        core_func_call(&self.name, base, &func_args)
    }
}



/*#[cfg(test)]
mod tests {
    use super::*;
    use runtime::{Scope, Value, VType};
    use self::VType::*;

    // ADD

    #[test]
    fn add_int_consts() {
        let mut state = Scope::new();
        let f = FuncMap::new();

        let add = AddExpr::new(Box::new(ValExpr::Int(25)), Box::new(ValExpr::Int(12)));

        assert_eq!(add.eval(&mut state, &f), Ok(Value::Val(I(37))));
    }

    #[test]
    fn add_int_to_float() {
        let mut state = Scope::new();
        let f = FuncMap::new();

        let add = AddExpr::new(Box::new(ValExpr::Int(25)), Box::new(ValExpr::Float(3.3)));

        assert_eq!(add.eval(&mut state, &f), Ok(Value::Val(F(28.3))));
    }

    #[test]
    fn add_int_to_text() {
        let mut state = Scope::new();
        let f = FuncMap::new();

        let add = AddExpr::new(Box::new(ValExpr::Int(25)), Box::new(ValExpr::Text(" twenty five".to_string())));

        let res = match add.eval(&mut state, &f) {
            Ok(Value::Str(s)) => s.borrow(),
            Err(e) => panic!("error: {}", e),
        };

        assert_eq!(res, "25 twenty five".to_string());
    }

    #[test]
    fn add_text_to_float() {
        let mut state = Scope::new();
        let f = FuncMap::new();

        let add = AddExpr::new(Box::new(ValExpr::Text("x = ".to_string())), Box::new(ValExpr::Float(3.3)));

        let res = match add.eval(&mut state, &f) {
            Ok(Value::Str(s)) => s.borrow(),
            Err(e) => panic!("error: {}", e),
        };

        assert_eq!(res, "x = 3.3".to_string());
    }
}
*/