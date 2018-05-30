// LALR Expression parser

use super::Token;
use parser::parser::get_package_ref;

use ast::*;
use runtime::Value;

use nom::{IResult, Needed, Err, ErrorKind, Context};

type ExprRes<'a> = IResult<&'a [Token], Box<Expr>>;

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
        Token::Times => match p_atom(&input[1..]) {
            Ok((ir, expr)) => {
                if ir.len() < 1 {
                    Err(Err::Incomplete(Needed::Size(1)))
                } else { match ir[0] {
                    Token::SemiColon |
                    Token::RPar |
                    Token::Comma => Ok((ir, Box::new(MulExpr::new(first,expr)))),
                    _ => p_op(Box::new(MulExpr::new(first,expr)), ir),
                }}
            },
            e => e,
        },
        Token::Divide => match p_atom(&input[1..]) {
            Ok((ir, expr)) => {
                if ir.len() < 1 {
                    Err(Err::Incomplete(Needed::Size(1)))
                } else { match ir[0] {
                    Token::SemiColon |
                    Token::RPar |
                    Token::Comma => Ok((ir, Box::new(DivExpr::new(first,expr)))),
                    _ => p_op(Box::new(DivExpr::new(first,expr)), ir),
                }}
            },
            e => e,
        },
        Token::Plus => match p_mul(&input[1..]) {
            Ok((ir, expr)) => {
                if ir.len() < 1 {
                    Err(Err::Incomplete(Needed::Size(1)))
                } else { match ir[0] {
                    Token::SemiColon |
                    Token::RPar |
                    Token::Comma => Ok((ir, Box::new(AddExpr::new(first,expr)))),
                    _ => p_op(Box::new(AddExpr::new(first,expr)), ir),
                }}
            },
            e => e,
        },
        Token::Minus => match p_mul(&input[1..]) {
            Ok((ir, expr)) => {
                if ir.len() < 1 {
                    Err(Err::Incomplete(Needed::Size(1)))
                } else { match ir[0] {
                    Token::SemiColon |
                    Token::RPar |
                    Token::Comma => Ok((ir, Box::new(SubExpr::new(first,expr)))),
                    _ => p_op(Box::new(SubExpr::new(first,expr)), ir),
                }}
            },
            e => e,
        },
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
                Token::Times => match p_atom(&ir[1..]) {
                    Ok((ir, expr)) => {
                        if ir.len() < 1 {
                            Err(Err::Incomplete(Needed::Size(1)))
                        } else { match ir[0] {
                            Token::SemiColon |
                            Token::RPar |
                            Token::Comma => Ok((ir, Box::new(MulExpr::new(first,expr)))),
                            _ => p_op(Box::new(MulExpr::new(first,expr)), ir),
                        }}
                    },
                    e => e,
                },
                Token::Divide => match p_atom(&ir[1..]) {
                    Ok((ir, expr)) => {
                        if ir.len() < 1 {
                            Err(Err::Incomplete(Needed::Size(1)))
                        } else { match ir[0] {
                            Token::SemiColon |
                            Token::RPar |
                            Token::Comma => Ok((ir, Box::new(DivExpr::new(first,expr)))),
                            _ => p_op(Box::new(DivExpr::new(first,expr)), ir),
                        }}
                    },
                    e => e,
                },
                _ => Ok((ir,first)),
            }}
        },
        e => e,
    }}
}
