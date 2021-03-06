use super::Token;
use super::resolver::Resolver;

use ast::*;
use error::{Error, Type, CompileCode};

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

use nom::{IResult, Needed, Err, ErrorKind, Context};


// For resolving context-specific package references
thread_local!(static RESOLVER: RefCell<Resolver> = RefCell::new(Resolver::new()));

pub fn get_package_ref(package_ref: Option<&str>) -> String {
    RESOLVER.with(|r| r.borrow()
                       .get_package_ref(package_ref)
                       .unwrap()
                 )
}

pub fn add_package_ref(package_ref: &str, package_name: &str) {
    RESOLVER.with(|r| r.borrow_mut().add_package_ref(package_ref, package_name));
}

pub fn add_local_ref(local_ref: &str, package_name: &str) {
    RESOLVER.with(|r| r.borrow_mut().add_local_ref(local_ref, package_name));
}

pub fn clear_local_refs() {
    RESOLVER.with(|r| r.borrow_mut().clear_local_refs());
}


pub fn parse_package(input: &[Token], name: &str) -> Result<ScriptPackage, Error> {
    RESOLVER.with(|r| r.borrow_mut().set_package(name));

    let mut output = match p_func_list(input) {
        Ok((_,o)) => o,
        Err(_) => return Err(Error::new(Type::CompileTime(CompileCode::Error))),
    };

    let mut package = BTreeMap::new();

    while let Some((n, f)) = output.pop() {
        package.insert(n, f);
    }

    RESOLVER.with(|r| r.borrow_mut().reset_package_refs());

    Ok(ScriptPackage::new(/*name, */package))
}

pub fn parse_snippet(input: &[Token], packs: &[(String, String)]) -> Result<Script, Error> {
    RESOLVER.with(|r| r.borrow_mut().set_package("0"));

    for &(ref n, ref r) in packs.iter() {
        add_package_ref(n,r);
    }

    let output = match p_stat(input) {
        Ok((_,o)) => o,
        Err(_) => return Err(Error::new(Type::CompileTime(CompileCode::Error))),
    };

    RESOLVER.with(|r| r.borrow_mut().reset_package_refs());

    Ok(Script::new(output))
}

pub fn parse_expr_snippet(input: &[Token], packs: &[(String, String)]) -> Result<ScriptExpr, Error> {
    RESOLVER.with(|r| r.borrow_mut().set_package("0"));

    for &(ref n, ref r) in packs.iter() {
        add_package_ref(n,r);
    }

    let output = match p_expr_snippet(input) {
        Ok((_,o)) => o,
        Err(_) => return Err(Error::new(Type::CompileTime(CompileCode::Error))),
    };

    RESOLVER.with(|r| r.borrow_mut().reset_package_refs());

    Ok(ScriptExpr::new(Some(output)))
}


named!(p_func_list<&[Token], Vec<(String, FuncRoot)> >,
    do_parse!(
        p_import    >>
        f: many1!(
            complete!(p_func)
        )           >>
        (f)
    )
);

named!(p_import<&[Token], Vec<()> >,
    many0!(
        do_parse!(
            apply!(compare, Token::Import)  >>
            alt!(
                do_parse!(
                    pack: is_str_lit                    >>
                    apply!(compare, Token::As)          >>
                    id: is_id                           >>
                    apply!(compare, Token::SemiColon)   >>
                    (add_package_ref(&id, &pack))
                )   |
                do_parse!(
                    pack: is_id                         >>
                    apply!(compare, Token::SemiColon)   >>
                    (add_package_ref(&pack, &pack))
                )   |
                do_parse!(
                    pack: is_id                         >>
                    apply!(compare, Token::As)          >>
                    id: is_id                           >>
                    apply!(compare, Token::SemiColon)   >>
                    (add_package_ref(&id, &pack))
                )
            )                               >>
            (())
        )
    )
);

named!(p_func<&[Token], (String, FuncRoot)>,
    do_parse!(
        apply!(compare, Token::Func)    >>
        n: is_id                        >>
        apply!(compare, Token::LPar)    >>
        a: p_id_list                    >>
        apply!(compare, Token::RPar)    >>
        apply!(compare, Token::LBrac)   >>
        c: p_func_body                  >>
        apply!(compare, Token::RBrac)   >>
        (n, FuncRoot::new(a,c))
    )
);

named!(p_id_list<&[Token], Vec<String> >,
    separated_list_complete!(
        apply!(compare, Token::Comma),
        is_id
    )
);

named!(p_func_body<&[Token], Vec<Box<dyn Statement> > >,
    do_parse!(
        p_func_imports  >>
        s: p_stat_list  >>
        end_function    >>  // TODO: remove this function
        (s)
    )
);

named!(p_func_imports<&[Token], Vec<()> >,
    many0!(
        do_parse!(
            apply!(compare, Token::Import)  >>
            alt!(
                do_parse!(
                    pack: is_str_lit                    >>
                    apply!(compare, Token::As)          >>
                    id: is_id                           >>
                    apply!(compare, Token::SemiColon)   >>
                    (add_local_ref(&id, &pack))
                )   |
                do_parse!(
                    pack: is_id                         >>
                    apply!(compare, Token::SemiColon)   >>
                    (add_local_ref(&pack, &pack))
                )   |
                do_parse!(
                    pack: is_id                         >>
                    apply!(compare, Token::As)          >>
                    id: is_id                           >>
                    apply!(compare, Token::SemiColon)   >>
                    (add_local_ref(&id, &pack))
                )
            )                               >>
            (())
        )
    )
);

named!(p_stat_list<&[Token], Vec<Box<dyn Statement> > >,
    many0!(
        p_stat
    )
);

named!(p_stat<&[Token], Box<dyn Statement> >,
    alt!(
        p_scope         |
        p_return_stat   |
        p_if_stat       |
        p_match_stat    |
        p_while_stat    |
        p_for_stat      |
        p_continue_stat |
        p_break_stat    |
        p_decl_stat     |
        p_assign_stat   |
        p_expr_stat
    )
);

named!(p_scope<&[Token], Box<dyn Statement> >,
    do_parse!(
        apply!(compare, Token::LBrac)   >>
        stats: p_stat_list              >>
        apply!(compare, Token::RBrac)   >>
        (Box::new(ScopeStat::new(stats)))
    )
);

named!(p_return_stat<&[Token], Box<dyn Statement> >,
    do_parse!(
        apply!(compare, Token::Return)      >>
        expr: opt!(p_expr)                  >>
        apply!(compare, Token::SemiColon)   >>
        (Box::new(ReturnStat::new(expr)))
    )
);

named!(p_if_stat<&[Token], Box<dyn Statement> >,
    do_parse!(
        apply!(compare, Token::If)  >>
        cond: p_expr                >>
        then: p_stat                >>
        elif: opt!(p_elif)          >>
        (Box::new(IfStat::new(cond, then, elif)))
    )
);

named!(p_elif<&[Token], Box<dyn Statement> >,
    alt!(
        do_parse!(
            apply!(compare, Token::Elif)    >>
            cond: p_expr                    >>
            then: p_stat                    >>
            elif: opt!(p_elif)              >>
            (Box::new(IfStat::new(cond, then, elif)) as Box<dyn Statement>)
        )   |
        do_parse!(
            apply!(compare, Token::Else)    >>
            then: p_stat                    >>
            (then)
        )
    )
);

named!(p_match_stat<&[Token], Box<dyn Statement> >,
    do_parse!(
        apply!(compare, Token::Match)   >>
        cond: p_expr                    >>
        apply!(compare, Token::LBrac)   >>
        cases: many1!(p_match_case)     >>
        otherwise: opt!(do_parse!(
            apply!(compare, Token::Else)    >>
            apply!(compare, Token::Colon)   >>
            stat: p_stat                    >>
            (stat)
        ))                              >>
        apply!(compare, Token::RBrac)   >>
        (Box::new(MatchStat::new(cond, cases, otherwise)))
    )
);

named!(p_match_case<&[Token], (CaseType, Box<dyn Statement>)>,
    do_parse!(
        t: alt!(
            do_parse!(
                id: is_id                       >>
                apply!(compare, Token::Colon)   >>
                (CaseType::Var(id))
            ) |
            do_parse!(
                e: p_expr                       >>
                apply!(compare, Token::Colon)   >>
                (CaseType::Value(e))
            )
        )               >>
        stat: p_stat    >>
        ((t, stat))
    )
);

named!(p_while_stat<&[Token], Box<dyn Statement> >,
    do_parse!(
        apply!(compare, Token::While)   >>
        cond: p_expr                    >>
        body: p_stat                    >>
        (Box::new(WhileStat::new(cond, body)))
    )
);

named!(p_for_stat<&[Token], Box<dyn Statement> >,
    do_parse!(
        apply!(compare, Token::For)     >>
        f: do_parse!(
            element: is_id              >>
            apply!(compare, Token::In)  >>
            list: p_expr                >>
            body: p_stat                >>
            (Box::new(ForStat::new(element, list, body)))
        )                               >>
        (f)
    )
);

named!(p_continue_stat<&[Token], Box<dyn Statement> >,
    do_parse!(
        apply!(compare, Token::Continue)    >>
        apply!(compare, Token::SemiColon)   >>
        (Box::new(ContinueStat::new()))
    )
);

named!(p_break_stat<&[Token], Box<dyn Statement> >,
    do_parse!(
        apply!(compare, Token::Break)       >>
        apply!(compare, Token::SemiColon)   >>
        (Box::new(BreakStat::new()))
    )
);

named!(p_decl_stat<&[Token], Box<dyn Statement> >,
    do_parse!(
        apply!(compare, Token::Var)         >>
        id: is_id                           >>
        a: opt!(do_parse!(
            apply!(compare, Token::Assign)  >>
            e: p_expr                       >>
            (e)
        ))                                  >>
        apply!(compare, Token::SemiColon)   >>
        (Box::new(VarDecl::new(&id, a)))
    )
);

named!(p_assign_stat<&[Token], Box<dyn Statement> >,
    do_parse!(
        e: complete!(p_expr_no_consume)     >>
        var: is_id                          >>
        child_op: opt!(p_assign_op_chain)   >>
        expr: apply!(assign_op, e)          >>
        apply!(compare, Token::SemiColon)   >>
        (Box::new(AssignStat::new(&var, expr, child_op)))
    )
);

named!(p_assign_op_chain<&[Token], Box<dyn Assign> >,
    alt!(
        do_parse!(
            apply!(compare, Token::LSq)         >>
            e: p_expr                           >>
            apply!(compare, Token::RSq)         >>
            child_op: opt!(p_assign_op_chain)   >>
            (Box::new(IndexAssign::new(e, child_op)) as Box<dyn Assign>)
        )   |
        do_parse!(
            apply!(compare, Token::Dot)         >>
            id: is_id                           >>
            child_op: opt!(p_assign_op_chain)   >>
            (Box::new(AccessAssign::new(&id, child_op)) as Box<dyn Assign>)
        )
    )
);

named!(p_expr_stat<&[Token], Box<dyn Statement> >,
    do_parse!(
        expr: complete!(p_expr)             >>
        apply!(compare, Token::SemiColon)   >>
        (Box::new(ExprStat::new(expr)))
    )
);

named!(pub p_expr_snippet<&[Token], Box<dyn Expr> >,
    do_parse!(
        expr: complete!(p_expr)             >>
        apply!(compare, Token::SemiColon)   >>
        (expr)
    )
);

named!(pub p_expr<&[Token], Box<dyn Expr> >,
    alt!(
        call!(super::expr::p_expr_lalr) |
        do_parse!(
            apply!(compare, Token::Func)    >>
            apply!(compare, Token::LPar)    >>
            a: p_id_list                    >>
            apply!(compare, Token::RPar)    >>
            apply!(compare, Token::LBrac)   >>
            c: p_func_body                  >>
            apply!(compare, Token::RBrac)   >>
            (Box::new(ValExpr::Closure( Rc::new(RefCell::new(FuncRoot::new(a,c))) )) as Box<dyn Expr>)
        )
    )
);

fn p_expr_no_consume<'a>(input: &'a [Token]) -> IResult<&'a [Token], Box<dyn Expr>> {
    match p_expr(&input[0..]) {
        Ok((_,res)) => Ok((&input[0..],res)),
        e => e,
    }
}

macro_rules! assign_expr {
    ($input:ident, $id:ident, $op_expr:path) => {
        match p_expr(&$input[1..]) {
            Ok((ir,expr)) => Ok((ir, Box::new($op_expr($id, expr)))),
            e => e,
        }
    };
}

fn assign_op<'a>(input: &'a [Token], id: Box<dyn Expr>) -> IResult<&'a [Token], Box<dyn Expr>> {
    if input.len() < 2 {
        Err(Err::Incomplete(Needed::Size(2)))
    } else { match input[0] {
        Token::Assign => p_expr(&input[1..]),
        Token::AsnPlus => assign_expr!(input, id, AddExpr::new),
        Token::AsnMinus => assign_expr!(input, id, SubExpr::new),
        Token::AsnTimes => assign_expr!(input, id, MulExpr::new),
        Token::AsnDivide => assign_expr!(input, id, DivExpr::new),
        Token::AsnModulo => assign_expr!(input, id, ModExpr::new),
        Token::AsnOr => assign_expr!(input, id, OrExpr::new),
        Token::AsnXor => assign_expr!(input, id, XorExpr::new),
        Token::AsnAnd => assign_expr!(input, id, AndExpr::new),
        _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(102)))),
    }}
}

fn compare(input: &[Token], t: Token) -> IResult<&[Token], &[Token]> {
    if input.len() == 0 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else if input[0] == t {
        Ok((&input[1..], &input[..0]))
    } else {
        Err(Err::Error(Context::Code(input, ErrorKind::Custom(100))))
    }
}

fn is_id(input: &[Token]) -> IResult<&[Token], String> {
    if input.len() == 0 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else {
        match input[0] {
            Token::Id(ref s) => Ok((&input[1..], s.clone())),
            _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(105)))),
        }
    }
}

fn is_str_lit(input: &[Token]) -> IResult<&[Token], String> {
    if input.len() == 0 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else {
        match input[0] {
            Token::StrLit(ref s) => Ok((&input[1..], s.clone())),
            _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(106)))),
        }
    }
}

fn end_function(input: &[Token]) -> IResult<&[Token], ()> {
    clear_local_refs();
    Ok((input, ()))
}


#[cfg(test)]
mod tests {
    use super::*;
    use runtime::{Value, FuncMap};
    use parser::tokeniser::tokenise;
    use VType::*;

    /*#[test]
    fn parse_function() {
        let input = "func f() {return 3;} func g() {return 5;}";
        let package_name = "root";
        let parsed = tokenise(input).unwrap();
        let package = parse_package(&parsed, package_name).unwrap();
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "f",
                              &Vec::new()),
                   Ok(Value::Val(I(3))));
    }*/

    #[test]
    fn parse_expr() {
        use runtime::Scope;

        let input = "3+3;";
        let parsed = tokenise(input).unwrap();

        assert_eq!(parsed, vec![Token::IntLit(3), Token::Plus, Token::IntLit(3), Token::SemiColon]);

        let (_, expr) = p_expr(&parsed).unwrap();

        let fm = FuncMap::new();
        let mut s = Scope::new();

        assert_eq!(expr.eval(&mut s, &fm).unwrap(), Value::Val(I(6)));
    }

    #[test]
    fn parse_rand_expr() {
        use runtime::Scope;

        let input = "(3+3)*5+33/5;";
        let parsed = tokenise(input).unwrap();

        //assert_eq!(parsed, vec![Token::IntLit(3), Token::Plus, Token::IntLit(3)]);

        let (_, expr) = p_expr(&parsed).unwrap();

        let fm = FuncMap::new();
        let mut s = Scope::new();

        assert_eq!(expr.eval(&mut s, &fm).unwrap(), Value::Val(I(36)));
    }

    #[test]
    fn parse_neg_expr() {
        use runtime::Scope;

        let input = "-3 * -5;";
        let parsed = tokenise(input).unwrap();

        //assert_eq!(parsed, vec![Token::IntLit(3), Token::Plus, Token::IntLit(3)]);

        let (_, expr) = p_expr(&parsed).unwrap();

        let fm = FuncMap::new();
        let mut s = Scope::new();

        assert_eq!(expr.eval(&mut s, &fm).unwrap(), Value::Val(I(15)));
    }

    #[test]
    fn parse_bool_expr() {
        use runtime::Scope;

        let input = "!false;";
        let parsed = tokenise(input).unwrap();

        //assert_eq!(parsed, vec![Token::IntLit(3), Token::Plus, Token::IntLit(3)]);

        let (_, expr) = p_expr(&parsed).unwrap();

        let fm = FuncMap::new();
        let mut s = Scope::new();

        assert_eq!(expr.eval(&mut s, &fm).unwrap(), Value::Val(B(true)));
    }

    #[test]
    fn parse_relate_expr() {
        use runtime::Scope;

        let input = "3 < 1 + 5;";
        let parsed = tokenise(input).unwrap();

        //assert_eq!(parsed, vec![Token::IntLit(3), Token::Plus, Token::IntLit(3)]);

        let (_, expr) = p_expr(&parsed).unwrap();

        let fm = FuncMap::new();
        let mut s = Scope::new();

        assert_eq!(expr.eval(&mut s, &fm).unwrap(), Value::Val(B(true)));
    }

    /*#[test]
    fn parse_child_function() {
        let input = "func f() {return c(2);} func c(x) {return x*2;}";
        let package_name = "root";
        let parsed = tokenise(input).unwrap();
        let package = parse_package(&parsed, package_name).unwrap();
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());
        let out = fm.call_fn("root", "f", &Vec::new());

        assert_eq!(out, Ok(Value::Val(I(4))));
    }*/

    #[test]
    fn parse_list() {
        let input = "x, y, z ";
        let parsed = tokenise(input).unwrap();

        assert_eq!(parsed, vec![Token::Id("x".to_string()), Token::Comma,
                                Token::Id("y".to_string()), Token::Comma,
                                Token::Id("z".to_string())]);

        let expected = vec!["x".to_string(), "y".to_string(), "z".to_string()];

        assert_eq!(p_id_list(&parsed), Ok((&Vec::new()[..], expected)));
    }
}
