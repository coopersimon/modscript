use super::Token;
use super::resolver::Resolver;

use ast::*;
//use runtime::Value;
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


pub fn parse_package(input: &[Token], name: &str) -> Result<ScriptPackage, String> {
    RESOLVER.with(|r| r.borrow_mut().set_package(name));

    let mut output = match p_func_list(input) {
        Ok((_,o)) => o,
        Err(e) => return Err(format!("Error: {:?}", e)),
    };

    let mut package = BTreeMap::new();

    while let Some((n, f)) = output.pop() {
        package.insert(n, f);
    }

    RESOLVER.with(|r| r.borrow_mut().reset_package_refs());

    Ok(ScriptPackage::new(/*name, */package))
}

// TODO: Anonymous functions here?
pub fn parse_snippet(input: &[Token], packs: &[(String, String)]) -> Result<Script, String> {
    RESOLVER.with(|r| r.borrow_mut().set_package("0"));

    for &(ref n, ref r) in packs.iter() {
        add_package_ref(n,r);
    }

    let output = match p_stat(input) {
        Ok((_,o)) => o,
        Err(e) => return Err(format!("Error: {:?}", e)),
    };

    RESOLVER.with(|r| r.borrow_mut().reset_package_refs());

    Ok(Script::new(output))
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

named!(p_func_body<&[Token], Vec<Box<Statement> > >,
    do_parse!(
        s: p_stat_list  >>
        (s)
    )
);

named!(p_stat_list<&[Token], Vec<Box<Statement> > >,
    many0!(
        p_stat
    )
);

named!(p_stat<&[Token], Box<Statement> >,
    alt!(
        p_scope         |
        p_return_stat   |
        p_if_stat       |
        p_while_stat    |
        p_for_stat      |
        p_continue_stat |
        p_break_stat    |
        p_decl_stat     |
        p_assign_stat   |
        p_expr_stat
    )
);

named!(p_scope<&[Token], Box<Statement> >,
    do_parse!(
        apply!(compare, Token::LBrac)   >>
        stats: p_stat_list              >>
        apply!(compare, Token::RBrac)   >>
        (Box::new(ScopeStat::new(stats)))
    )
);

named!(p_return_stat<&[Token], Box<Statement> >,
    do_parse!(
        apply!(compare, Token::Return)      >>
        expr: opt!(p_expr)                  >>
        apply!(compare, Token::SemiColon)   >>
        (Box::new(ReturnStat::new(expr)))
    )
);

named!(p_if_stat<&[Token], Box<Statement> >,
    do_parse!(
        apply!(compare, Token::If)  >>
        cond: p_expr                >>
        then: p_stat                >>
        elif: opt!(p_elif)          >>
        (Box::new(IfStat::new(cond, then, elif)))
    )
);

named!(p_elif<&[Token], Box<Statement> >,
    alt!(
        do_parse!(
            apply!(compare, Token::Elif)    >>
            cond: p_expr                    >>
            then: p_stat                    >>
            elif: opt!(p_elif)              >>
            (Box::new(IfStat::new(cond, then, elif)) as Box<Statement>)
        )   |
        do_parse!(
            apply!(compare, Token::Else)    >>
            then: p_stat                    >>
            (then)
        )
    )
);

named!(p_while_stat<&[Token], Box<Statement> >,
    do_parse!(
        apply!(compare, Token::While)   >>
        cond: p_expr                    >>
        body: p_stat                    >>
        (Box::new(WhileStat::new(cond, body)))
    )
);

named!(p_for_stat<&[Token], Box<Statement> >,
    do_parse!(
        apply!(compare, Token::For)     >>
        f: alt!(
            do_parse!(
                element: is_id              >>
                apply!(compare, Token::In)  >>
                list: p_expr                >>
                body: p_stat                >>
                (Box::new(ForStat::new(element, list, body)) as Box<Statement>)
            )   |
            do_parse!(
                apply!(compare, Token::LPar)        >>
                init: p_stat                        >>
                apply!(compare, Token::SemiColon)   >>
                cond: p_expr                        >>
                apply!(compare, Token::SemiColon)   >>
                end: p_stat                         >>
                apply!(compare, Token::RPar)        >>
                body: p_stat                        >>
                (Box::new(LoopStat::new(init, cond, end, body)) as Box<Statement>)
            )
        )                               >>
        (f)
    )
);

named!(p_continue_stat<&[Token], Box<Statement> >,
    do_parse!(
        apply!(compare, Token::Continue)    >>
        apply!(compare, Token::SemiColon)   >>
        (Box::new(ContinueStat::new()))
    )
);

named!(p_break_stat<&[Token], Box<Statement> >,
    do_parse!(
        apply!(compare, Token::Break)       >>
        apply!(compare, Token::SemiColon)   >>
        (Box::new(BreakStat::new()))
    )
);

named!(p_decl_stat<&[Token], Box<Statement> >,
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

named!(p_assign_stat<&[Token], Box<Statement> >,
    do_parse!(
        e: complete!(p_expr_no_consume)     >>
        var: is_id                          >>
        child_op: opt!(p_assign_op_chain)   >>
        expr: apply!(assign_op, e)          >>
        apply!(compare, Token::SemiColon)   >>
        (Box::new(AssignStat::new(&var, expr, child_op)))
    )
);

named!(p_assign_op_chain<&[Token], Box<Assign> >,
    alt!(
        do_parse!(
            apply!(compare, Token::LSq)         >>
            e: p_expr                           >>
            apply!(compare, Token::RSq)         >>
            child_op: opt!(p_assign_op_chain)   >>
            (Box::new(IndexAssign::new(e, child_op)) as Box<Assign>)
        )   |
        do_parse!(
            apply!(compare, Token::Dot)         >>
            id: is_id                           >>
            child_op: opt!(p_assign_op_chain)   >>
            (Box::new(AccessAssign::new(&id, child_op)) as Box<Assign>)
        )
    )
);

named!(p_expr_stat<&[Token], Box<Statement> >,
    do_parse!(
        expr: complete!(p_expr)             >>
        apply!(compare, Token::SemiColon)   >>
        (Box::new(ExprStat::new(expr)))
    )
);

named!(p_expr<&[Token], Box<Expr> >,
    alt!(
        call!(super::expr::p_expr) |
        do_parse!(
            apply!(compare, Token::Func)    >>
            apply!(compare, Token::LPar)    >>
            a: p_id_list                    >>
            apply!(compare, Token::RPar)    >>
            apply!(compare, Token::LBrac)   >>
            c: p_func_body                  >>
            apply!(compare, Token::RBrac)   >>
            (Box::new(ValExpr::Closure( Rc::new(RefCell::new(FuncRoot::new(a,c))) )) as Box<Expr>)
        )
    )
);

fn p_expr_no_consume<'a>(input: &'a [Token]) -> IResult<&'a [Token], Box<Expr>> {
    match super::expr::p_expr(&input[0..]) {
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

fn assign_op<'a>(input: &'a [Token], id: Box<Expr>) -> IResult<&'a [Token], Box<Expr>> {
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


#[cfg(test)]
mod tests {
    use super::*;
    use runtime::{Value, VType, FuncMap};
    use parser::tokeniser::tokenise;
    use VType::*;

    #[test]
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
    }

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

    #[test]
    fn parse_child_function() {
        let input = "func f() {return c(2);} func c(x) {return x*2;}";
        let package_name = "root";
        let parsed = tokenise(input).unwrap();
        let package = parse_package(&parsed, package_name).unwrap();
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());
        let out = fm.call_fn("root", "f", &Vec::new());

        assert_eq!(out, Ok(Value::Val(I(4))));
    }

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
