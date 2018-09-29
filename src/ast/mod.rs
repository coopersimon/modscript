// scripting engine ast
mod stat;
mod expr;
mod assign;
mod root;

pub use self::stat::*;
pub use self::expr::*;
pub use self::assign::*;
pub use self::root::*;

use runtime::{Value, Scope, ExprRes, Signal, FuncMap};

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

pub trait Assign: AstNode {
    fn assign(&self, Value, Value, &mut Scope, &FuncMap) -> Signal;
}
