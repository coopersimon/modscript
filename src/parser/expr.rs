// LALR Expression parser

use super::Token;
use parser::parser::get_package_ref;

use ast::*;
use runtime::Value;

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
                    Token::Comma => Ok((ir, Box::new($ast($first,expr)))),
                    _ => p_op(Box::new($ast($first,expr)), ir),
                }}
            },
            e => e,
        }
    };
}

/*pub fn p_expr(input: &[Token]) -> ExprRes {
    if input.len() < 2 {
        Err(Err::Incomplete(Needed::Size(2)))
    } else match input[0] {
        Token::IntLit(i) => match input[1] {
            Token::SemiColon |
            Token::RPar |
            Token::Comma => Ok((&input[1..], Box::new(ValExpr::Int(i)))),
            _ => p_op(Box::new(ValExpr::Int(i)), &input[1..]),
        },
        Token::LPar => match p_expr(&input[1..]) {
            Ok((ir, expr)) => {
                if ir.len() < 2 {
                    Err(Err::Incomplete(Needed::Size(2)))
                } else match (ir[0], ir[1]) {
                    (Token::RPar, Token::SemiColon) |
                    (Token::RPar, Token::RPar) |
                    (Token::RPar, Token::Comma) => Ok((&ir[1..], expr)),
                    (Token::RPar, _) => p_op(expr, &ir[1..]),
                    (_, _) => Err(Err::Error(Context::Code(ir, ErrorKind::Custom(100)))),
                }
            },
            e => e,
        },
        Token::Id(ref n) => match input[1] {
            Token::SemiColon |
            Token::RPar |
            Token::Comma => Ok((&input[1..], Box::new(ValExpr::Var(n.clone())))),
            Token::LPar => p_func_call(n, &input[1..]),
            _ => p_op(Box::new(ValExpr::Var(n.clone())), &input[1..]),
        },
    }
}*/

pub fn p_expr<'a>(input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 2 {
        Err(Err::Incomplete(Needed::Size(2)))
    } else { match p_atom(&input) {
        Ok((ir,expr)) => {
            if ir.len() < 1 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else { match ir[0] {
                Token::SemiColon |
                Token::RPar |
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
        Token::Times => op_match!(input, p_atom, first, MulExpr::new),
        Token::Divide => op_match!(input, p_atom, first, DivExpr::new),
        Token::Plus => op_match!(input, p_mul, first, AddExpr::new),
        Token::Minus => op_match!(input, p_mul, first, SubExpr::new),
        _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(101)))),
    }}
}

// Assuming input[0] has already been matched as LPar
fn p_func_call<'a>(name: &String, input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 3 {
        Err(Err::Incomplete(Needed::Size(3)))
    } else { match (&input[1], &input[2]) {
        (&Token::RPar, &Token::SemiColon) |
        (&Token::RPar, &Token::RPar) |
        (&Token::RPar, &Token::Comma) => Ok((&input[2..], Box::new(FuncCall::new(&get_package_ref(None),name,Vec::new())))),
        (&Token::RPar, _) => p_op(Box::new(FuncCall::new(&get_package_ref(None),name,Vec::new())),&input[2..]),
        (_,_) => match p_arg_list(&input[0..], Vec::new()) {
            Ok((ir,args)) => {
                if ir.len() < 1 {
                    Err(Err::Incomplete(Needed::Size(1)))
                } else { match ir[0] {
                    Token::SemiColon |
                    Token::RPar |
                    Token::Comma => Ok((ir, Box::new(FuncCall::new(&get_package_ref(None),name,args)))),
                    _ => p_op(Box::new(FuncCall::new(&get_package_ref(None),name,args)),ir),
                }}
            },
            Err(e) => Err(e),
        },
    }}
}

// Assuming input[0] has already been matched as LPar or comma
fn p_arg_list<'a>(input: &'a [Token], mut args: Vec<Box<Expr>>) -> IResult<&'a [Token], Vec<Box<Expr>>> {
    if input.len() < 3 {
        Err(Err::Incomplete(Needed::Size(3)))
    } else { match p_expr(&input[1..]) {
        Ok((ir,arg)) => {
            args.push(arg);
            if ir.len() < 1 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else { match ir[0] {
                Token::Comma => p_arg_list(ir, args),
                Token::RPar => Ok((&ir[1..], args)),
                _ => Err(Err::Error(Context::Code(ir, ErrorKind::Custom(101)))),
            }}
        },
        Err(e) => Err(e),
    }}
}

fn p_atom<'a>(input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 2 {
        Err(Err::Incomplete(Needed::Size(2)))
    } else { match input[0] {
        Token::IntLit(i) => Ok((&input[1..], Box::new(ValExpr::Int(i)))),
        Token::LPar => match p_expr(&input[1..]) {
            Ok((ir, expr)) => {
                if ir.len() < 2 {
                    Err(Err::Incomplete(Needed::Size(2)))
                } else { match ir[0] {
                    Token::RPar => Ok((&ir[1..], expr)),
                    _ => Err(Err::Error(Context::Code(ir, ErrorKind::Custom(100)))),
                }}
            },
            e => e,
        },
        Token::Id(ref n) => match input[1] {
            Token::LPar => p_func_call(n, &input[1..]),
            _ => Ok((&input[1..], Box::new(ValExpr::Var(n.clone())))),
        },
        _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(100)))),
    }}
}

fn p_mul<'a>(input: &'a [Token]) -> ExprRes<'a> {
    if input.len() < 1 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else { match p_atom(&input) {
        Ok((ir,first)) => {
            if ir.len() < 1 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else { match ir[0] {
                Token::Times => op_match!(ir, p_atom, first, MulExpr::new),
                Token::Divide => op_match!(ir, p_atom, first, DivExpr::new),
                _ => Ok((ir,first)),
            }}
        },
        e => e,
    }}
}
