// LALR Expression parser

use super::Token;
use parser::parser::get_package_ref;

use ast::*;

use nom::{IResult, Needed, Err, ErrorKind, Context};

type ExprRes<'a> = IResult<&'a [Token], Box<Expr>>;

macro_rules! op_match {
    ($input:ident, $mat:ident, $first:expr, $ast:path) => {
        match $mat(&$input[1..]) {
            Ok((ir, expr)) => {
                if ir.len() < 1 {
                    Err(Err::Incomplete(Needed::Size(1)))
                } else { match ir[0] {
                    Token::SemiColon    |
                    Token::RPar         |
                    Token::LBrac        |
                    Token::RBrac        |
                    Token::RSq          |
                    Token::AsnPlus      |
                    Token::AsnMinus     |
                    Token::AsnTimes     |
                    Token::AsnDivide    |
                    Token::AsnModulo    |
                    Token::AsnOr        |
                    Token::AsnXor       |
                    Token::AsnAnd       |
                    Token::Assign       |
                    Token::Comma => Ok((ir, Box::new($ast($first,expr)))),
                    _ => p_op(Box::new($ast($first,expr)), ir),
                }}
            },
            e => e,
        }
    };
}

pub fn p_expr<'a>(input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 2 {
        Err(Err::Incomplete(Needed::Size(2)))
    } else { match p_unary(&input) {
        Ok((ir,expr)) => {
            if ir.len() < 1 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else { match ir[0] {
                Token::SemiColon    |
                Token::RPar         |
                Token::LBrac        |
                Token::RBrac        |
                Token::RSq          |
                Token::AsnPlus      |
                Token::AsnMinus     |
                Token::AsnTimes     |
                Token::AsnDivide    |
                Token::AsnModulo    |
                Token::AsnOr        |
                Token::AsnXor       |
                Token::AsnAnd       |
                Token::Assign       |
                Token::Comma => Ok((ir,expr)),
                _ => p_op(expr, ir),
            }}
        },
        e => e,
    }}
}

fn p_op<'a>(first: Box<Expr>, input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 2 {
        Err(Err::Incomplete(Needed::Size(2)))
    } else { match input[0] {
        // Token::LSq...
        Token::Times => op_match!(input, p_unary, first, MulExpr::new),
        Token::Divide => op_match!(input, p_unary, first, DivExpr::new),
        Token::Modulo => op_match!(input, p_unary, first, ModExpr::new),
        Token::Plus => op_match!(input, p_mul_div, first, AddExpr::new),
        Token::Minus => op_match!(input, p_mul_div, first, SubExpr::new),
        Token::GThan => op_match!(input, p_add_sub, first, GThanExpr::new),
        Token::GEq => op_match!(input, p_add_sub, first, GEqExpr::new),
        Token::LThan => op_match!(input, p_add_sub, first, LThanExpr::new),
        Token::LEq => op_match!(input, p_add_sub, first, LEqExpr::new),
        Token::Equal => op_match!(input, p_relational, first, EqExpr::new),
        Token::NEqual => op_match!(input, p_relational, first, NEqExpr::new),
        Token::TrueEq => op_match!(input, p_relational, first, TrueEqExpr::new),
        Token::TrueNEq => op_match!(input, p_relational, first, TrueNEqExpr::new),
        Token::And => op_match!(input, p_equality, first, AndExpr::new),
        Token::Xor => op_match!(input, p_equality, first, XorExpr::new),
        Token::Or => op_match!(input, p_equality, first, OrExpr::new),
        _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(101)))),
    }}
}

// Assuming input[0] has already been matched as LPar or comma or LSq
fn p_expr_list<'a>(input: &'a [Token], mut exprs: Vec<Box<Expr>>, term: Token) -> IResult<&'a [Token], Vec<Box<Expr>>> {
    if input.len() < 3 {
        Err(Err::Incomplete(Needed::Size(3)))
    } else { match p_expr(&input[1..]) {
        Ok((ir,expr)) => {
            exprs.push(expr);
            if ir.len() < 2 {
                Err(Err::Incomplete(Needed::Size(2)))
            } else { match ir[0] {
                Token::Comma => p_expr_list(ir, exprs, term),
                ref t if t == &term => Ok((&ir[1..], exprs)),
                _ => Err(Err::Error(Context::Code(ir, ErrorKind::Custom(101)))),
            }}
        },
        Err(e) => Err(e),
    }}
}

// Assuming input[0] has already been matched as Dot
fn p_access<'a>(input: &'a [Token], first: Box<Expr>) -> ExprRes<'a> {
    if input.len() < 3 {
        Err(Err::Incomplete(Needed::Size(3)))
    } else { match input[1] {
        Token::Id(ref n) => p_post_op(&input[2..], Box::new(AccessExpr::new(first,n))),
        _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(100)))),
    }}
}

// Assuming input[0] has already been matched as Arrow
fn p_core_func<'a>(input: &'a [Token], first: Box<Expr>) -> ExprRes<'a> {
    if input.len() < 5 {
        Err(Err::Incomplete(Needed::Size(5)))
    } else { match (&input[1], &input[2], &input[3]) {
        (&Token::Id(ref n), &Token::LPar, &Token::RPar) => p_post_op(&input[4..],Box::new(CoreFuncCall::new(n,first,Vec::new()))),
        (&Token::Id(ref n), &Token::LPar, _) => match p_expr_list(&input[2..],Vec::new(),Token::RPar) {
            Ok((ir,args)) => p_post_op(ir,Box::new(CoreFuncCall::new(n,first,args))),
            Err(e) => Err(e),
        },
        (_,_,_) => Err(Err::Error(Context::Code(input, ErrorKind::Custom(100)))),
    }}
}

fn p_post_op<'a>(input: &'a [Token], first: Box<Expr>) -> ExprRes<'a> {
    if input.len() < 1 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else { match input[0] {
        Token::LPar => {
            if input.len() < 2 {
                Err(Err::Incomplete(Needed::Size(2)))
            } else { match input[1] {
                Token::RPar => p_post_op(&input[2..], Box::new(FuncCall::new(first, Vec::new()))),
                _ => match p_expr_list(&input[0..], Vec::new(), Token::RPar) {
                    Ok((ir, args)) => p_post_op(ir, Box::new(FuncCall::new(first, args))),
                    Err(e) => Err(e),
                },
            }}
        },
        Token::LSq => {
            if input.len() < 2 {
                Err(Err::Incomplete(Needed::Size(2)))
            } else { match p_expr(&input[1..]) {
                Ok((ir,expr)) => if ir.len() < 2 {
                    Err(Err::Incomplete(Needed::Size(2)))
                } else { match ir[0] {
                    Token::RSq => p_post_op(&ir[1..],Box::new(IndexExpr::new(first,expr))),
                    _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(100)))),
                }},
                e => e,
            }}
        },
        Token::Arrow => p_core_func(input, first),
        Token::Dot => p_access(input, first),
        _ => Ok((&input, first)),
    }}
}

fn p_pair<'a>(input: &'a[Token], split: Token) -> IResult<&'a [Token], (String, Box<Expr>)> {
    if input.len() < 3 {
        Err(Err::Incomplete(Needed::Size(3)))
    } else { match (&input[0],input[1] == split) {
        (&Token::Id(ref n), true) => match p_expr(&input[2..]) {
            Ok((ir,expr)) => Ok((ir,(n.clone(),expr))),
            Err(e) => Err(e),
        },
        (_,_) => Err(Err::Error(Context::Code(input, ErrorKind::Custom(100)))),
    }}
}

fn p_object<'a>(input: &'a[Token], mut items: Vec<(String, Box<Expr>)>) -> ExprRes<'a> {
    if input.len() < 2 {
        Err(Err::Incomplete(Needed::Size(2)))
    } else { match input[0] {
        Token::RBrac => Ok((&input[1..], Box::new(ValExpr::Obj(items)))),
        _ => match p_pair(&input[0..], Token::Colon) {
            Ok((ir,res)) => {
                items.push(res);
                if ir.len() < 2 {
                    Err(Err::Incomplete(Needed::Size(2)))
                } else { match ir[0] {
                    Token::RBrac => Ok((&ir[1..], Box::new(ValExpr::Obj(items)))),
                    Token::Comma => p_object(&ir[1..], items),
                    _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(100)))),
                }}
            },
            Err(e) => Err(e),
        },
    }}
}

fn p_atom<'a>(input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 2 {
        Err(Err::Incomplete(Needed::Size(2)))
    } else { match input[0] {
        Token::IntLit(i) => p_post_op(&input[1..], Box::new(ValExpr::Int(i))),
        Token::FloatLit(f) => p_post_op(&input[1..], Box::new(ValExpr::Float(f))),
        Token::StrLit(ref s) => p_post_op(&input[1..], Box::new(ValExpr::Text(s.clone()))),
        Token::True => Ok((&input[1..], Box::new(ValExpr::Bool(true)))),    //TODO: can you post-op true/false?
        Token::False => Ok((&input[1..], Box::new(ValExpr::Bool(false)))),
        Token::LPar => match p_expr(&input[1..]) {
            Ok((ir,expr)) => {
                if ir.len() < 2 {
                    Err(Err::Incomplete(Needed::Size(2)))
                } else { match ir[0] {
                    Token::RPar => p_post_op(&ir[1..], expr),
                    _ => Err(Err::Error(Context::Code(ir, ErrorKind::Custom(100)))),
                }}
            },
            e => e,
        },
        Token::LSq => match input[1] {
            Token::RSq => p_post_op(&input[2..], Box::new(ValExpr::List(Vec::new()))), // TODO: check size
            _ => match p_expr_list(&input, Vec::new(), Token::RSq) {
                Ok((ir,exprs)) => p_post_op(ir, Box::new(ValExpr::List(exprs))),
                Err(e) => Err(e),
            },
        },
        Token::LBrac => p_object(&input[1..], Vec::new()),
        Token::Id(ref n) => match input[1] {
            Token::DoubleColon => {
                if input.len() < 4 {
                    Err(Err::Incomplete(Needed::Size(4)))
                } else { match input[2] {
                    Token::Id(ref n2) => p_post_op(&input[3..], Box::new(ValExpr::QualId(get_package_ref(Some(n)), n2.clone()))),
                    _ => Err(Err::Error(Context::Code(&input[2..], ErrorKind::Custom(100)))),
                }}
            },
            _ => p_post_op(&input[1..], Box::new(ValExpr::Id(n.clone()))),
        },
        Token::Ref => match input[1] {
            Token::Id(ref n) => p_post_op(&input[2..], Box::new(ValExpr::Ref(n.clone()))), // TODO: check size
            _ => Err(Err::Error(Context::Code(&input[1..], ErrorKind::Custom(100)))),
        },
        _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(100)))),
    }}
}

fn p_unary<'a>(input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 2 {
        Err(Err::Incomplete(Needed::Size(2)))
    } else { match input[0] {
        Token::Not => match p_atom(&input[1..]) {
            Ok((ir,expr)) => Ok((ir, Box::new(NotExpr::new(expr)))),
            e => e,
        },
        Token::Minus => match p_atom(&input[1..]) {
            Ok((ir,expr)) => Ok((ir, Box::new(NegExpr::new(expr)))),
            e => e,
        },
        _ => p_atom(&input),
    }}
}

fn p_mul_div<'a>(input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 1 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else { match p_unary(&input) {
        Ok((ir,first)) => {
            if ir.len() < 1 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else { match ir[0] {
                Token::Times => op_match!(ir, p_unary, first, MulExpr::new),
                Token::Divide => op_match!(ir, p_unary, first, DivExpr::new),
                Token::Modulo => op_match!(ir, p_unary, first, ModExpr::new),
                _ => Ok((ir,first)),
            }}
        },
        e => e,
    }}
}

fn p_add_sub<'a>(input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 1 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else { match p_unary(&input) {
        Ok((ir,first)) => {
            if ir.len() < 1 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else { match ir[0] {
                Token::Times => op_match!(ir, p_unary, first, MulExpr::new),
                Token::Divide => op_match!(ir, p_unary, first, DivExpr::new),
                Token::Modulo => op_match!(ir, p_unary, first, ModExpr::new),
                Token::Plus => op_match!(ir, p_mul_div, first, AddExpr::new),
                Token::Minus => op_match!(ir, p_mul_div, first, SubExpr::new),
                _ => Ok((ir,first)),
            }}
        },
        e => e,
    }}
}

fn p_relational<'a>(input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 1 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else { match p_unary(&input) {
        Ok((ir,first)) => {
            if ir.len() < 1 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else { match ir[0] {
                Token::Times => op_match!(ir, p_unary, first, MulExpr::new),
                Token::Divide => op_match!(ir, p_unary, first, DivExpr::new),
                Token::Modulo => op_match!(ir, p_unary, first, ModExpr::new),
                Token::Plus => op_match!(ir, p_mul_div, first, AddExpr::new),
                Token::Minus => op_match!(ir, p_mul_div, first, SubExpr::new),
                Token::GThan => op_match!(ir, p_add_sub, first, GThanExpr::new),
                Token::GEq => op_match!(ir, p_add_sub, first, GEqExpr::new),
                Token::LThan => op_match!(ir, p_add_sub, first, LThanExpr::new),
                Token::LEq => op_match!(ir, p_add_sub, first, LEqExpr::new),
                _ => Ok((ir,first)),
            }}
        },
        e => e,
    }}
}

fn p_equality<'a>(input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 1 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else { match p_unary(&input) {
        Ok((ir,first)) => {
            if ir.len() < 1 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else { match ir[0] {
                Token::Times => op_match!(ir, p_unary, first, MulExpr::new),
                Token::Divide => op_match!(ir, p_unary, first, DivExpr::new),
                Token::Modulo => op_match!(ir, p_unary, first, ModExpr::new),
                Token::Plus => op_match!(ir, p_mul_div, first, AddExpr::new),
                Token::Minus => op_match!(ir, p_mul_div, first, SubExpr::new),
                Token::GThan => op_match!(ir, p_add_sub, first, GThanExpr::new),
                Token::GEq => op_match!(ir, p_add_sub, first, GEqExpr::new),
                Token::LThan => op_match!(ir, p_add_sub, first, LThanExpr::new),
                Token::LEq => op_match!(ir, p_add_sub, first, LEqExpr::new),
                Token::Equal => op_match!(input, p_relational, first, EqExpr::new),
                Token::NEqual => op_match!(input, p_relational, first, NEqExpr::new),
                Token::TrueEq => op_match!(input, p_relational, first, TrueEqExpr::new),
                Token::TrueNEq => op_match!(input, p_relational, first, TrueNEqExpr::new),
                _ => Ok((ir,first)),
            }}
        },
        e => e,
    }}
}
