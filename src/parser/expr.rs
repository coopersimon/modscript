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
                    Token::SemiColon |
                    Token::RPar |
                    Token::LBrac |
                    Token::RSq |
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
                Token::SemiColon |
                Token::RPar |
                Token::LBrac |
                Token::RSq |
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

fn p_path<'a>(package: &String, input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 2 {
        Err(Err::Incomplete(Needed::Size(2)))
    } else { match input[0] {
        // TODO: check for fn call vs variable
        Token::Id(ref n) => p_func_call(n, package, &input[1..]),
        _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(101)))),
    }}
}

// Assuming input[0] has already been matched as LPar
fn p_func_call<'a>(name: &String, package: &String, input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 3 {
        Err(Err::Incomplete(Needed::Size(3)))
    } else { match (&input[1], &input[2]) {
        (&Token::RPar, &Token::SemiColon) |
        (&Token::RPar, &Token::RPar) |
        (&Token::RPar, &Token::LBrac) |
        (&Token::RPar, &Token::RSq) |
        (&Token::RPar, &Token::Comma) => Ok((&input[2..], Box::new(FuncCall::new(package,name,Vec::new())))),
        (&Token::RPar, _) => p_op(Box::new(FuncCall::new(package,name,Vec::new())),&input[2..]),
        (_,_) => match p_expr_list(&input[0..], Vec::new(), Token::RPar) {
            Ok((ir,args)) => {
                if ir.len() < 1 {
                    Err(Err::Incomplete(Needed::Size(1)))
                } else { match ir[0] {
                    Token::SemiColon |
                    Token::RPar |
                    Token::LBrac |
                    Token::RSq |
                    Token::Comma => Ok((ir, Box::new(FuncCall::new(package,name,args)))),
                    _ => p_op(Box::new(FuncCall::new(package,name,args)),ir),
                }}
            },
            Err(e) => Err(e),
        },
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

fn p_post_op<'a>(input: &'a [Token], first: Box<Expr>) -> ExprRes<'a> {
    if input.len() < 1 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else { match input[0] {
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
        //Token::Arrow
        _ => Ok((&input, first))
    }}
}

fn p_atom<'a>(input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 2 {
        Err(Err::Incomplete(Needed::Size(2)))
    } else { match input[0] {
        Token::IntLit(i) => Ok((&input[1..], Box::new(ValExpr::Int(i)))),
        Token::FloatLit(f) => Ok((&input[1..], Box::new(ValExpr::Float(f)))),
        Token::StrLit(ref s) => Ok((&input[1..], Box::new(ValExpr::Text(s.clone())))),
        Token::True => Ok((&input[1..], Box::new(ValExpr::Bool(true)))),
        Token::False => Ok((&input[1..], Box::new(ValExpr::Bool(false)))),
        Token::LPar => match p_expr(&input[1..]) {
            Ok((ir,expr)) => {
                if ir.len() < 2 {
                    Err(Err::Incomplete(Needed::Size(2)))
                } else { match ir[0] {
                    Token::RPar => Ok((&ir[1..], expr)),
                    _ => Err(Err::Error(Context::Code(ir, ErrorKind::Custom(100)))),
                }}
            },
            e => e,
        },
        Token::LSq => match input[1] {
            Token::RSq => Ok((&input[2..], Box::new(ValExpr::List(Vec::new())))), // TODO: check size
            _ => match p_expr_list(&input, Vec::new(), Token::RSq) {
                Ok((ir,exprs)) => Ok((ir, Box::new(ValExpr::List(exprs)))),
                Err(e) => Err(e),
            },
        },
        Token::Id(ref n) => match input[1] {
            /*Token::LSq => {
                if input.len() < 3 {
                    Err(Err::Incomplete(Needed::Size(3)))
                } else { match p_expr(&input[2..]) {
                    Ok((ir,expr)) => {
                        if ir.len() < 2 {
                            Err(Err::Incomplete(Needed::Size(100)))
                        } else { match ir[0] {
                            Token::RSq => Ok((&ir[1..],Box::new(IndexExpr::new(n.clone(),expr)))),
                            _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(100)))),
                        }}
                    },
                    e => e,
                }}
            },*/
            Token::LPar => p_func_call(n, &get_package_ref(None), &input[1..]),
            Token::DoubleColon => p_path(&get_package_ref(Some(n)), &input[2..]), // TODO: check size, avoid clone
            //_ => Ok((&input[1..], Box::new(ValExpr::Var(n.clone())))),
            _ => p_post_op(&input[1..], Box::new(ValExpr::Var(n.clone()))),
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
