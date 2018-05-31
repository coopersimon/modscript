use super::Token;
use super::resolver::Resolver;

use ast::*;
use runtime::Value;
use std::cell::RefCell;

use std::collections::BTreeMap;

use nom::{IResult, Needed, Err, ErrorKind, Context};


// For resolving context-specific package references
thread_local!(static RESOLVER: RefCell<Resolver> = RefCell::new(Resolver::new()));

pub fn get_package_ref(package_ref: Option<String>) -> String {
    RESOLVER.with(|r| r.borrow()
                       .get_package_ref(package_ref)
                       .unwrap()
                 )
}

pub fn add_package_ref(package_ref: &str, package_name: &str) {
    RESOLVER.with(|r| r.borrow_mut().add_package_ref(package_ref, package_name));
}


pub fn parse(input: &[Token], name: &str) -> Result<ScriptPackage, String> {
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

    Ok(ScriptPackage::new(package))
}


named!(p_func_list<&[Token], Vec<(String, FuncRoot)> >,
    many1!(
        complete!(p_func)
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
        // imports
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
        //p_func_stat     |
        p_return_stat   |
        //p_if_stat       |
        p_while_stat    |
        p_decl_stat     |
        p_assign_stat
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

/*named!(p_if_stat<&[Token], Box<Statement> >,
    do_parse!(
        apply!(compare, Token::If)  >>
        cond: p_expr                >>
        then: p_stat                >>
        // elif
        (Box::new)
    )
);*/

named!(p_while_stat<&[Token], Box<Statement> >,
    do_parse!(
        apply!(compare, Token::While)   >>
        cond: p_expr                    >>
        body: p_stat                    >>
        (Box::new(WhileStat::new(cond, body)))
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
        var: is_id                          >>
        //expr: apply!(p_assign_expr, &var)   >>
        expr: do_parse!(
            apply!(compare, Token::Assign)  >>
            e: p_expr                       >>
            (e)
        )                                   >>
        apply!(compare, Token::SemiColon)   >>
        (Box::new(AssignStat::new(&var, expr)))
    )
);

/*named_args!(p_assign_expr(var: &str)<&[Token], Box<Expr> >,
    switch!(take!(1),
        Token::Assign   => p_expr    |
        Token::AsnPlus  => do_parse!(
            e: p_expr
            (Box::new(AddExpr::new(Box::new(ValExpr(var.to_string())),e)))
        )                           //|
    )
);*/

named!(p_expr<&[Token], Box<Expr> >,
    //call!(p_or)
    call!(super::expr::p_expr)
);

/*named!(p_or<&[Token], Box<Expr> >,
    alt!(
        p_xor |
        do_parse!(
            a: p_xor                                >>
            complete!(apply!(compare, Token::Or))   >>
            b: p_or                                 >>
            (Box::new(OrExpr::new(a,b)) as Box<Expr>)
        )
    )
);

named!(p_xor<&[Token], Box<Expr> >,
    alt!(
        p_and |
        do_parse!(
            a: p_and                    >>
            complete!(apply!(compare, Token::Xor)) >>
            b: p_xor                    >>
            (Box::new(XorExpr::new(a,b)) as Box<Expr>)
        )
    )
);

named!(p_and<&[Token], Box<Expr> >,
    alt!(
        p_equals |
        do_parse!(
            a: p_equals                 >>
            complete!(apply!(compare, Token::And)) >>
            b: p_and                    >>
            (Box::new(AndExpr::new(a,b)) as Box<Expr>)
        )
    )
);

named!(p_equals<&[Token], Box<Expr> >,
    alt!(
        p_eq        |
        p_neq       |
        p_true_eq   |
        p_true_neq  |
        p_relational
    )
);

named!(p_eq<&[Token], Box<Expr> >,
    do_parse!(
        a: p_relational                 >>
        complete!(apply!(compare, Token::Equal)  ) >>
        b: p_equals                     >>
        (Box::new(EqExpr::new(a,b)))
    )
);

named!(p_neq<&[Token], Box<Expr> >,
    do_parse!(
        a: p_relational                 >>
        complete!(apply!(compare, Token::NEqual) ) >>
        b: p_equals                     >>
        (Box::new(NotExpr::new(Box::new(EqExpr::new(a,b)))))
    )
);

named!(p_true_eq<&[Token], Box<Expr> >,
    do_parse!(
        a: p_relational                 >>
        complete!(apply!(compare, Token::TrueEq) ) >>
        b: p_equals                     >>
        (Box::new(TrueEqExpr::new(a,b)))
    )
);

named!(p_true_neq<&[Token], Box<Expr> >,
    do_parse!(
        a: p_relational                 >>
        complete!(apply!(compare, Token::TrueNEq)) >>
        b: p_equals                     >>
        (Box::new(NotExpr::new(Box::new(TrueEqExpr::new(a,b)))))
    )
);

named!(p_relational<&[Token], Box<Expr> >,
    alt!(
        p_gthan     |
        p_geq       |
        p_lthan     |
        p_leq       |
        p_add_sub
    )
);

named!(p_gthan<&[Token], Box<Expr> >,
    do_parse!(
        a: p_add_sub                    >>
        complete!(apply!(compare, Token::GThan)  ) >>
        b: p_relational                 >>
        (Box::new(GThanExpr::new(a,b)))
    )
);

named!(p_geq<&[Token], Box<Expr> >,
    do_parse!(
        a: p_add_sub                    >>
        complete!(apply!(compare, Token::GEq)    ) >>
        b: p_relational                 >>
        (Box::new(GEqExpr::new(a,b)))
    )
);

named!(p_lthan<&[Token], Box<Expr> >,
    do_parse!(
        a: p_add_sub                    >>
        complete!(apply!(compare, Token::LThan)  ) >>
        b: p_relational                 >>
        (Box::new(LThanExpr::new(a,b)))
    )
);

named!(p_leq<&[Token], Box<Expr> >,
    do_parse!(
        a: p_add_sub                    >>
        complete!(apply!(compare, Token::LEq)    ) >>
        b: p_relational                 >>
        (Box::new(LEqExpr::new(a,b)))
    )
);

named!(p_add_sub<&[Token], Box<Expr> >,
    alt!(
        p_add   |
        p_sub   |
        p_mul_div
    )
);

named!(p_add<&[Token], Box<Expr> >,
    do_parse!(
        a: p_mul_div                    >>
        complete!(apply!(compare, Token::Plus)   ) >>
        b: p_add_sub                    >>
        (Box::new(AddExpr::new(a,b)))
    )
);

named!(p_sub<&[Token], Box<Expr> >,
    do_parse!(
        a: p_mul_div                    >>
        complete!(apply!(compare, Token::Minus)  ) >>
        b: p_add_sub                    >>
        (Box::new(SubExpr::new(a,b)))
    )
);

named!(p_mul_div<&[Token], Box<Expr> >,
    alt!(
        p_mul   |
        p_div   |
        p_mod   |
        p_func_expr
    )
);

named!(p_mul<&[Token], Box<Expr> >,
    do_parse!(
        a: p_func_expr                  >>
        complete!(apply!(compare, Token::Times)  ) >>
        b: p_mul_div                    >>
        (Box::new(MulExpr::new(a,b)))
    )
);

named!(p_div<&[Token], Box<Expr> >,
    do_parse!(
        a: p_func_expr                  >>
        complete!(apply!(compare, Token::Divide) ) >>
        b: p_mul_div                    >>
        (Box::new(DivExpr::new(a,b)))
    )
);

named!(p_mod<&[Token], Box<Expr> >,
    do_parse!(
        a: p_func_expr                  >>
        complete!(apply!(compare, Token::Modulo) ) >>
        b: p_mul_div                    >>
        (Box::new(ModExpr::new(a,b)))
    )
);

named!(p_func_expr<&[Token], Box<Expr> >,
    alt!(
        p_not       |
        p_func_call |
        p_prim_expr
    )
);

named!(p_func_call<&[Token], Box<Expr> >,
    do_parse!(
        // package
        id: is_id                       >>
        apply!(compare, Token::LPar)    >>
        args: p_expr_list               >>
        apply!(compare, Token::RPar)    >>
        (Box::new(FuncCall::new(&get_package_ref(None), &id, args)))
    )
);

named!(p_expr_list<&[Token], Vec<Box<Expr> > >,
    separated_list_complete!(
        apply!(compare, Token::Comma),
        p_expr
        // or, "ref id"
    )
);

named!(p_not<&[Token], Box<Expr> >,
    do_parse!(
        apply!(compare, Token::Not) >>
        e: p_prim_expr              >>
        (Box::new(NotExpr::new(e)))
    )
);

named!(p_prim_expr<&[Token], Box<Expr> >,
    alt!(
        p_float_expr    |
        p_int_expr      |
        p_text_expr     |
        p_bool_expr     |
        p_id_expr       |
        p_par_expr
    )
);

named!(p_float_expr<&[Token], Box<Expr> >,
    do_parse!(
        f: is_float >>
        (Box::new(ValExpr::Float(f)))
    )
);

named!(p_int_expr<&[Token], Box<Expr> >,
    do_parse!(
        i: is_int   >>
        (Box::new(ValExpr::Int(i)))
    )
);

named!(p_text_expr<&[Token], Box<Expr> >,
    do_parse!(
        t: is_str_lit >>
        (Box::new(ValExpr::Text(t)))
    )
);

named!(p_bool_expr<&[Token], Box<Expr> >,
    alt!(
        do_parse!(
            apply!(compare, Token::True)    >>
            (Box::new(ValExpr::Bool(true)) as Box<Expr>)
        )   |
        do_parse!(
            apply!(compare, Token::False)   >>
            (Box::new(ValExpr::Bool(false)) as Box<Expr>)
        )
    )
);

named!(p_id_expr<&[Token], Box<Expr> >,
    do_parse!(
        id: is_id   >>
        (Box::new(ValExpr::Var(id)))
    )
);

named!(p_par_expr<&[Token], Box<Expr> >,
    do_parse!(
        apply!(compare, Token::LPar)    >>
        expr: p_expr                    >>
        apply!(compare, Token::RPar)    >>
        (expr)
    )
);*/


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
            _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(101)))),
        }
    }
}

fn is_int(input: &[Token]) -> IResult<&[Token], i64> {
    if input.len() == 0 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else {
        match input[0] {
            Token::IntLit(i) => Ok((&input[1..], i)),
            _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(101)))),
        }
    }
}

fn is_float(input: &[Token]) -> IResult<&[Token], f64> {
    if input.len() == 0 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else {
        match input[0] {
            Token::FloatLit(f) => Ok((&input[1..], f)),
            _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(101)))),
        }
    }
}

fn is_str_lit(input: &[Token]) -> IResult<&[Token], String> {
    if input.len() == 0 {
        Err(Err::Incomplete(Needed::Size(1)))
    } else {
        match input[0] {
            Token::StrLit(ref s) => Ok((&input[1..], s.clone())),
            _ => Err(Err::Error(Context::Code(input, ErrorKind::Custom(101)))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use runtime::{FuncMap, ExprRes};
    use parser::tokeniser::tokenise;

    #[test]
    fn parse_function() {
        let input = "func f() {return 3;} func g() {return 5;}";
        let package_name = "root";
        let parsed = tokenise(input).unwrap();
        let package = parse(&parsed, package_name).unwrap();
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "f",
                              &Vec::new()),
                   Ok(Value::Int(3)));
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

        assert_eq!(expr.eval(&mut s, &fm).unwrap(), Value::Int(6));
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

        assert_eq!(expr.eval(&mut s, &fm).unwrap(), Value::Int(36));
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

        assert_eq!(expr.eval(&mut s, &fm).unwrap(), Value::Int(15));
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

        assert_eq!(expr.eval(&mut s, &fm).unwrap(), Value::Bool(true));
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

        assert_eq!(expr.eval(&mut s, &fm).unwrap(), Value::Bool(true));
    }

    #[test]
    fn parse_child_function() {
        use std::time::{Duration, SystemTime};

        //let time = SystemTime::now();

        let input = "func f() {return c(2);} func c(x) {return x*2;}";
        let package_name = "root";
        let parsed = tokenise(input).unwrap();
        //eprintln!("Elapsed: {:?}", time.elapsed());
        let package = parse(&parsed, package_name).unwrap();
        //eprintln!("Elapsed: {:?}", time.elapsed());
        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());
        let out = fm.call_fn("root", "f", &Vec::new());
        //eprintln!("Elapsed: {:?}", time.elapsed());

        assert_eq!(out, Ok(Value::Int(4)));
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
