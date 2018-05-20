mod resolver;

use ast::*;
use runtime::Value;

use self::resolver::Resolver;

use nom::{multispace, double, digit, is_alphanumeric};
use nom::IResult;

use std::str;
use std::cell::RefCell;
use std::collections::BTreeMap;

use std::time::{Duration, SystemTime};


// For resolving context-specific package refrences
thread_local!(static RESOLVER: RefCell<Resolver> = RefCell::new(Resolver::new()));

fn get_package_ref(package_ref: Option<String>) -> String {
    RESOLVER.with(|r| r.borrow()
                       .get_package_ref(package_ref)
                       .unwrap()
                 )
}

fn add_package_ref(package_ref: &str, package_name: &str) {
    RESOLVER.with(|r| r.borrow_mut().add_package_ref(package_ref, package_name));
}


// TOP LEVEL
pub fn parse_package(name: &str, text: &str) -> Result<ScriptPackage, String> {
    let now = SystemTime::now();
    println!("Adding package {}", name);
    RESOLVER.with(|r| r.borrow_mut().set_package(name));

    let mut output = match p_func_list(text.as_bytes()) {
        IResult::Done(_,o) => o,
        IResult::Incomplete(i) => return Err(format!("Incomplete: {:?}", i)),
        IResult::Error(e) => return Err(format!("Error: {:?}", e)),
    };

    // convert vec into map
    let mut package = BTreeMap::new();

    while let Some((n, f)) = output.pop() {
        package.insert(n, f);
    }

    RESOLVER.with(|r| r.borrow_mut().reset_package_refs());

    let t = now.elapsed();

    eprintln!("Elpased: {:?}", t);

    Ok(ScriptPackage::new(package))
}

pub fn parse_snippet(imports: &[(String, String)], text: &str) -> Result<Script, String> {
    let now = SystemTime::now();
    for &(ref r,ref n) in imports {
        add_package_ref(r, n);
    }
    // setup imports
    RESOLVER.with(|r| r.borrow_mut().set_package(""));

    let t = now.elapsed();

    eprintln!("Elpased: {:?}", t);

    let output = match p_stat(text.as_bytes()) {
        IResult::Done(_,o) => o,
        IResult::Incomplete(i) => return Err(format!("Incomplete: {:?}", i)),
        IResult::Error(e) => return Err(format!("Error: {:?}", e)),
    };

    RESOLVER.with(|r| r.borrow_mut().reset_package_refs());

    eprintln!("Elpased: {:?}", now.elapsed());

    Ok(Script::new(output))
}


// TOKENS
const FUNC: &'static str = "func";
const VAR: &'static str = "var";
const RETURN: &'static str = "return";
const FOR: &'static str = "for";
const WHILE: &'static str = "while";
const IF: &'static str = "if";
const ELSE: &'static str = "else";
// continue
// break
// in
const TRUE: &'static str = "true";
const FALSE: &'static str = "false";


// PARSER

named!(p_func_list<&[u8], Vec<(String, FuncRoot)> >,
    many1!(
        do_parse!(
            f: p_func                       >>
            opt!(alt_complete!(multispace)) >>
            (f)
        )
    )
);

named!(p_func<&[u8], (String, FuncRoot)>,
    do_parse!(
        tag!(FUNC)              >>
        multispace              >>
        name: p_id              >>
        opt!(multispace)        >>
        tag!("(")               >>
        opt!(multispace)        >>
        args: opt!(p_id_list)   >>
        opt!(multispace)        >>
        tag!(")")               >>
        opt!(multispace)        >>
        body: p_func_body       >>
        (name, FuncRoot::new(args, body))
    )
);

named!(p_id<&[u8], String>,
    map_res!(p_id_chars, make_id)
);

named!(p_id_chars<&[u8], &[u8]>,
    take_while!(
        |c| {is_alphanumeric(c) || (c == 0x5F)}
    )
);

named!(p_id_list<&[u8], Vec<String> >,
    do_parse!(
        args: many0!(
            do_parse!(
                arg: p_id           >>
                opt!(multispace)    >>
                tag!(",")           >>
                opt!(multispace)    >>
                (arg)
            )
        )               >>
        final_arg: p_id >>
        (combine_list(args, Some(final_arg)))
    )
);

named!(p_func_body<&[u8], Vec<Box<Statement> > >,
    do_parse!(
        tag!("{")           >>
        opt!(multispace)    >>
        stats: p_stat_list  >>
        tag!("}")           >>
        (stats)
    )
);

named!(p_stat_list<&[u8], Vec<Box<Statement> > >,
    many0!(
        do_parse!(
            s: p_stat           >>
            opt!(multispace)    >>
            (s)
        )
    )
);

named!(p_stat<&[u8], Box<Statement> >,
    alt!(
        p_scope         |
        p_return_stat   |
        p_if_stat       |
        //p_for_stat       |
        p_while_stat    |
        p_decl_stat     |
        p_assign_stat
    )
);

named!(p_scope<&[u8], Box<Statement> >,
    do_parse!(
        tag!("{")                   >>
        opt!(multispace)            >>
        stats: opt!(p_stat_list)    >>
        tag!("}")                   >>
        (Box::new(ScopeStat::new(stats)))
    )
);

named!(p_return_stat<&[u8], Box<Statement> >,
    do_parse!(
        tag!(RETURN)        >>
        e: opt!(do_parse!(
            multispace  >>
            e: p_expr   >>
            (e)
        ))                  >>
        opt!(multispace)    >>
        tag!(";")           >>
        (Box::new(ReturnStat::new(e)))
    )
);

named!(p_if_stat<&[u8], Box<Statement> >,
    do_parse!(
        tag!(IF)            >>
        cond: p_wrap_expr   >>
        //opt!(multispace)    >>
        then: p_stat        >>
        e: opt!(
            do_parse!(
                opt!(multispace)    >>
                tag!(ELSE)          >>
                s: p_stat           >>
                (s)
            )
        )                   >>
        (Box::new(IfStat::new(cond, then, e)))
    )
);

named!(p_while_stat<&[u8], Box<Statement> >,
    do_parse!(
        tag!(WHILE)         >>
        c: p_wrap_expr      >>
        opt!(multispace)    >>
        b: p_stat           >>
        (Box::new(WhileStat::new(c, b)))
    )
);

named!(p_wrap_expr<&[u8], Box<Expr> >,
    alt!(
        do_parse!(
            opt!(multispace)    >>
            tag!("(")           >>
            opt!(multispace)    >>
            e: p_expr           >>
            opt!(multispace)    >>
            tag!(")")           >>
            opt!(multispace)    >>
            (e)
        )   |
        do_parse!(
            multispace  >>
            e: p_expr   >>
            multispace  >>
            (e)
        )
    )
);

named!(p_decl_stat<&[u8], Box<Statement> >,
    do_parse!(
        tag!(VAR)           >>
        multispace          >>
        v: p_id             >>
        opt!(multispace)    >>
        a: opt!(p_assign)   >>
        opt!(multispace)    >>
        tag!(";")           >>
        (Box::new(VarDecl::new(&v,a)))
    )
);

named!(p_assign<&[u8], Box<Expr> >,
    do_parse!(
        tag!("=")           >>
        opt!(multispace)    >>
        e: p_expr           >>
        (e)
    )
);

named!(p_assign_stat<&[u8], Box<Statement> >,
    do_parse!(
        v: p_id             >>
        opt!(multispace)    >>
        e: p_assign         >>
        opt!(multispace)    >>
        tag!(";")           >>
        (Box::new(AssignStat::new(&v,e)))
    )
);

named!(p_expr<&[u8], Box<Expr> >,
    alt!(
        p_or
    )
);

named!(p_or<&[u8], Box<Expr> >,
    alt!(
        do_parse!(
            a: p_xor            >>
            opt!(multispace)    >>
            tag!("|")           >>
            opt!(multispace)    >>
            b: p_or             >>
            (Box::new(OrExpr::new(a, b)) as Box<Expr>)
        )   |
        p_xor
    )
);

named!(p_xor<&[u8], Box<Expr> >,
    alt!(
        do_parse!(
            a: p_and            >>
            opt!(multispace)    >>
            tag!("^")           >>
            opt!(multispace)    >>
            b: p_xor            >>
            (Box::new(XorExpr::new(a, b)) as Box<Expr>)
        )   |
        p_and
    )
);

named!(p_and<&[u8], Box<Expr> >,
    alt!(
        do_parse!(
            a: p_equals         >>
            opt!(multispace)    >>
            tag!("&")           >>
            opt!(multispace)    >>
            b: p_and            >>
            (Box::new(AndExpr::new(a, b)) as Box<Expr>)
        )   |
        p_equals
    )
);

named!(p_equals<&[u8], Box<Expr> >,
    alt!(
        p_eq        |
        p_neq       |
        p_true_eq   |
        p_true_neq  |
        p_relational
    )
);

named!(p_eq<&[u8], Box<Expr> >,
    do_parse!(
        a: p_relational     >>
        opt!(multispace)    >>
        tag!("==")          >>
        opt!(multispace)    >>
        b: p_equals         >>
        (Box::new(EqExpr::new(a, b)))
    )
);

named!(p_neq<&[u8], Box<Expr> >,
    do_parse!(
        a: p_relational     >>
        opt!(multispace)    >>
        tag!("!=")          >>
        opt!(multispace)    >>
        b: p_equals         >>
        (Box::new(NotExpr::new(Box::new(EqExpr::new(a, b)))))
    )
);

named!(p_true_eq<&[u8], Box<Expr> >,
    do_parse!(
        a: p_relational     >>
        opt!(multispace)    >>
        tag!("===")         >>
        opt!(multispace)    >>
        b: p_equals         >>
        (Box::new(TrueEqExpr::new(a, b)))
    )
);

named!(p_true_neq<&[u8], Box<Expr> >,
    do_parse!(
        a: p_relational     >>
        opt!(multispace)    >>
        tag!("!==")         >>
        opt!(multispace)    >>
        b: p_equals         >>
        (Box::new(NotExpr::new(Box::new(TrueEqExpr::new(a, b)))))
    )
);

named!(p_relational<&[u8], Box<Expr> >,
    alt!(
        p_gthan |
        p_geq   |
        p_lthan |
        p_leq   |
        p_add_sub
    )
);

named!(p_gthan<&[u8], Box<Expr> >,
    do_parse!(
        a: p_add_sub        >>
        opt!(multispace)    >>
        tag!(">")           >>
        opt!(multispace)    >>
        b: p_relational     >>
        (Box::new(GThanExpr::new(a, b)))
    )
);

named!(p_geq<&[u8], Box<Expr> >,
    do_parse!(
        a: p_add_sub        >>
        opt!(multispace)    >>
        tag!(">=")          >>
        opt!(multispace)    >>
        b: p_relational     >>
        (Box::new(GEqExpr::new(a, b)))
    )
);

named!(p_lthan<&[u8], Box<Expr> >,
    do_parse!(
        a: p_add_sub        >>
        opt!(multispace)    >>
        tag!("<")           >>
        opt!(multispace)    >>
        b: p_relational     >>
        (Box::new(LThanExpr::new(a, b)))
    )
);

named!(p_leq<&[u8], Box<Expr> >,
    do_parse!(
        a: p_add_sub        >>
        opt!(multispace)    >>
        tag!("<=")          >>
        opt!(multispace)    >>
        b: p_relational     >>
        (Box::new(LEqExpr::new(a, b)))
    )
);

named!(p_add_sub<&[u8], Box<Expr> >,
    alt!(
        p_add   |
        p_sub   |
        p_mul_div
    )
);

named!(p_add<&[u8], Box<Expr> >,
    do_parse!(
        a: p_mul_div        >>
        opt!(multispace)    >>
        tag!("+")           >>
        opt!(multispace)    >>
        b: p_add_sub        >>
        (Box::new(AddExpr::new(a, b)))
    )
);

named!(p_sub<&[u8], Box<Expr> >,
    do_parse!(
        a: p_mul_div        >>
        opt!(multispace)    >>
        tag!("-")           >>
        opt!(multispace)    >>
        b: p_add_sub        >>
        (Box::new(SubExpr::new(a, b)))
    )
);

named!(p_mul_div<&[u8], Box<Expr> >,
    alt!(
        p_mul   |
        p_div   |
        p_mod   |
        p_func_expr
    )
);

named!(p_mul<&[u8], Box<Expr> >,
    do_parse!(
        a: p_func_expr      >>
        opt!(multispace)    >>
        tag!("*")           >>
        opt!(multispace)    >>
        b: p_mul_div        >>
        (Box::new(MulExpr::new(a, b)))
    )
);

named!(p_div<&[u8], Box<Expr> >,
    do_parse!(
        a: p_func_expr      >>
        opt!(multispace)    >>
        tag!("/")           >>
        opt!(multispace)    >>
        b: p_mul_div        >>
        (Box::new(DivExpr::new(a, b)))
    )
);

named!(p_mod<&[u8], Box<Expr> >,
    do_parse!(
        a: p_func_expr      >>
        opt!(multispace)    >>
        tag!("%")           >>
        opt!(multispace)    >>
        b: p_mul_div        >>
        (Box::new(ModExpr::new(a, b)))
    )
);

named!(p_func_expr<&[u8], Box<Expr> >,
    alt!(
        p_not       |
        p_func_call |
        p_prim_expr
    )
);

named!(p_func_call<&[u8], Box<Expr> >,
    do_parse!(
        //p: opt!(p_package)      >>
        f: p_id                 >>
        tag!("(")               >>
        opt!(multispace)        >>
        args: opt!(p_expr_list) >>
        opt!(multispace)        >>
        tag!(")")               >>
        (Box::new(FuncCall::new(&get_package_ref(Some("pack".to_string())), &f, args)))
    )
);

named!(p_package<&[u8], String>,
    do_parse!(
        n: alt!(
            p_id    |
            p_str_lit
        )           >>
        tag!("::")  >>
        (n)
    )
);

named!(p_str_lit<&[u8], String>,
    alt!(
        do_parse!(
            tag!("\"")              >>
            s: take_until!("\"")    >>
            tag!("\"")              >>
            (String::from_utf8(s.to_vec()).unwrap())
        )   |
        do_parse!(
            tag!("\'")              >>
            s: take_until!("\'")    >>
            tag!("\'")              >>
            (String::from_utf8(s.to_vec()).unwrap())
        )
    )
);

named!(p_not<&[u8], Box<Expr> >,
    do_parse!(
        tag!("!")           >>
        opt!(multispace)    >>
        e: p_prim_expr      >>
        (Box::new(NotExpr::new(e)))
    )
);

named!(p_prim_expr<&[u8], Box<Expr> >,
    alt!(
        p_float_expr    |
        p_int_expr      |
        p_text_expr     |
        p_bool_expr     |
        p_id_expr       |
        p_par_expr
    )
);

named!(p_float_expr<&[u8], Box<Expr> >,
    do_parse!(
        f: double   >>
        (Box::new(ValExpr::Float(f)))
    )
);

named!(p_int_expr<&[u8], Box<Expr> >,
    do_parse!(
        i: digit    >>
        (Box::new(ValExpr::Int(str_to_int(i).unwrap())))
    )
);

named!(p_text_expr<&[u8], Box<Expr> >,
    do_parse!(
        s: p_str_lit    >>
        (Box::new(ValExpr::Text(s)) as Box<Expr>)
    )
);

named!(p_bool_expr<&[u8], Box<Expr> >,
    alt!(
        do_parse!(
            tag!(TRUE)  >>
            (Box::new(ValExpr::Bool(true)) as Box<Expr>)
        )   |
        do_parse!(
            tag!(FALSE)  >>
            (Box::new(ValExpr::Bool(false)) as Box<Expr>)
        )
    )
);

named!(p_id_expr<&[u8], Box<Expr> >,
    do_parse!(
        id: p_id    >>
        (Box::new(ValExpr::Var(id)))
    )
);

named!(p_par_expr<&[u8], Box<Expr> >,
    do_parse!(
        tag!("(")           >>
        opt!(multispace)    >>
        e: p_expr           >>
        opt!(multispace)    >>
        tag!(")")           >>
        (e)
    )
);

named!(p_expr_list<&[u8], Vec<Box<Expr> > >,
    do_parse!(
        exprs: many0!(
            do_parse!(
                e: p_expr           >>
                opt!(multispace)    >>
                tag!(",")           >>
                opt!(multispace)    >>
                (e)
            )
        )                           >>
        final_expr: p_expr          >>
        (combine_list(exprs, Some(final_expr)))
    )
);


fn make_id(id: &[u8]) -> Result<String, String> {
    let s = str::from_utf8(id).unwrap();

    /*let s = match end {
        Some(s) => out.to_string() + str::from_utf8(s).unwrap(),
        None => out.to_string()
    };*/

    match s.chars().next() {
        Some(c) => if !c.is_alphabetic() {
            return Err(format!("Id begins with non-letter."));
        },
        None => return Err(format!("Id has no characters.")),
    }
    
    // check id isn't a keyword.
    match s {
        FUNC    => Err(format!("{} matches keyword", s)),
        VAR     => Err(format!("{} matches keyword", s)),
        RETURN  => Err(format!("{} matches keyword", s)),
        FOR     => Err(format!("{} matches keyword", s)),
        WHILE   => Err(format!("{} matches keyword", s)),
        IF      => Err(format!("{} matches keyword", s)),
        ELSE    => Err(format!("{} matches keyword", s)),
        TRUE    => Err(format!("{} matches keyword", s)),
        FALSE   => Err(format!("{} matches keyword", s)),
        _ => Ok(s.to_string()),
    }
}

fn combine_list<T>(mut args: Vec<T>, final_arg: Option<T>) -> Vec<T> {
    match final_arg {
        Some(a) => args.push(a),
        None => {},
    }
    args
}

fn str_to_int(s: &[u8]) -> Result<i64, String> {
    match str::from_utf8(s) {
        Ok(i_str) => match i_str.parse::<i64>() {
            Ok(i) => Ok(i),
            Err(_) => Err(format!("Not an integer: {}", i_str)),
        },
        Err(_) => Err(format!("Incorrectly parsed input string.")),
    }
}

/*fn debug_print(i: &[u8]) -> Result<&[u8], String> {
    eprintln!("got");
    Ok(i)
}*/


#[cfg(test)]
mod tests {
    use super::*;
    use runtime::{FuncMap, ExprRes};

    #[test]
    fn parse_single_func() {
        let func = "func f(){}";

        let p = parse_package("p", func).unwrap();

        assert!(p.funcs.contains_key("f"));

        let f = p.funcs.get("f").unwrap();

        assert_eq!(f.get_arg_names(), Vec::<String>::new().as_slice());
    }

    #[test]
    fn parse_func_arg() {
        let func = "func f(x){}";

        let p = parse_package("p", func).unwrap();

        let f = p.funcs.get("f").unwrap();

        //assert_eq!(s, "f".to_string());
        assert_eq!(f.get_arg_names(), (vec!["x".to_string()]).as_slice());
    }

    #[test]
    fn parse_func_args() {
        let func = "func f(a, b){}";

        //let (s,f) = parse_func(func);
        let p = parse_package("p", func).unwrap();

        let f = p.funcs.get("f").unwrap();

        //assert_eq!(s, "f".to_string());
        assert_eq!(f.get_arg_names(), (vec!["a".to_string(), "b".to_string()]).as_slice());
    }

    #[test]
    fn parse_func_body() {
        let func = "func f() {return;}";

        let p = parse_package("p", func).unwrap();

        let f = p.funcs.get("f").unwrap();

        let fm = FuncMap::new();

        //assert_eq!(s, "f".to_string());
        assert_eq!(f.get_arg_names(), Vec::<String>::new().as_slice());

        // run func, check it returns null
        assert_eq!(f.call(&Vec::new(), &fm), Ok(Value::Null));
    }

    #[test]
    fn parse_func_body_and_attach() {
        let func = "func f() {return;}";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        // run func, check it returns null
        assert_eq!(fm.call_fn("root",
                              "f",
                              &Vec::new()),
                   Ok(Value::Null));
    }

    // expr tests
    #[test]
    fn parse_func_with_return_expr() {
        let func = "func f() {return 3+2;}";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "f",
                              &Vec::new()),
                   Ok(Value::Int(5)));
    }

    #[test]
    fn parse_func_with_precedence() {
        let func = "func f() {return 3+2*5;}";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "f",
                              &Vec::new()),
                   Ok(Value::Int(13)));
    }

    #[test]
    fn parse_func_with_logical_ops() {
        let func = "func f() {return true & false;}";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "f",
                              &Vec::new()),
                   Ok(Value::Bool(false)));
    }

    #[test]
    fn parse_func_with_logical_precedence() {
        let func = "func f() {return true | false & true;}";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "f",
                              &Vec::new()),
                   Ok(Value::Bool(true)));
    }

    #[test]
    fn parse_func_with_bitwise_ops() {
        let func = "func f() {return 3 | 4;}";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "f",
                              &Vec::new()),
                   Ok(Value::Int(7)));
    }

    #[test]
    fn parse_func_with_return_from_args() {
        let func = "func add(a,b) {return a+b;}";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "add",
                              &vec![Value::Int(3),Value::Int(2)]),
                   Ok(Value::Int(5)));
    }

    #[test]
    fn parse_func_with_string() {
        let func = "func hello(n) {return 'Hello ' + n;}";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "hello",
                              &vec![Value::Int(3)]),
                   Ok(Value::Str("Hello 3".to_string())));
    }

    // This test is a bit gratuitous
    #[test]
    fn parse_func_with_return_args_expr_cast() {
        let func = "func add(a,b) {return a+b+1.1;}";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "add",
                              &vec![Value::Float(1.1),Value::Float(1.0)]),
                   Ok(Value::Float(3.2)));
    }

    // func calls
    #[test]
    fn parse_funcs_with_func_call() {
        let func = "func parent(x){return child(x+1) + x;} func child(x){return x*2;}";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "parent",
                              &vec![Value::Int(3)]),
                   Ok(Value::Int(11)));
    }

    // if test
    #[test]
    fn parse_func_with_if() {
        let func = "func zero(x) {if x return false; return true;}";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "zero",
                              &vec![Value::Int(0)]),
                   Ok(Value::Bool(true)));
    }
    
    #[test]
    fn parse_func_with_bool_if() {
        let func = "func gthan(x,y){ if x > y return true; return false; }";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "gthan",
                              &vec![Value::Int(5), Value::Float(3.3)]),
                   Ok(Value::Bool(true)));
    }

    // recursive
    #[test]
    fn parse_recursive_func() {
        //let func = "func fact(x) { if x > 1 return x*fact(x-1); return 1;}";
        let func = "func even_or(x) {if (x % 2 == 1) {return x;} return even_or(x + 1); }";
        //let func = "func even_or(x){if (x % 2 == 1) return x; return x+1; }";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "even_or",
                              &vec![Value::Int(4)]),
                   Ok(Value::Int(5)));
    }

    // declaring and using variables
    #[test]
    fn parse_func_with_local_variables() {
        let func = "func f(x) {var a = 3+x; return a*2;}";
        let package_name = "root";

        let package = parse_package(package_name, func).unwrap();

        let mut fm = FuncMap::new();

        fm.attach_package(package_name, package.call_ref());

        assert_eq!(fm.call_fn("root",
                              "f",
                              &vec![Value::Str("dupe".to_string())]),
                   Ok(Value::Str("3dupe3dupe".to_string())));
    }
}
