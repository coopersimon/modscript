use super::Token;
use error::{Error, Type, CompileCode};

use nom::{multispace, alphanumeric, alpha, digit};

pub fn tokenise(input: &str) -> Result<Vec<Token>, Error> {
    match p_token_list(input) {
        Ok((_,o)) => Ok(o),
        //Err(e) => Err(format!("Error: {:?}", e)),
        Err(_) => Err(Error::new(Type::CompileTime(CompileCode::Error))),
    }
}


// TOKENS
const FUNC: &'static str = "func";
const VAR: &'static str = "var";
const RETURN: &'static str = "return";
const FOR: &'static str = "for";
const WHILE: &'static str = "while";
const IN: &'static str = "in";
const IF: &'static str = "if";
const ELIF: &'static str = "elif";
const ELSE: &'static str = "else";
const CONTINUE: &'static str = "continue";
const BREAK: &'static str = "break";
const TRUE: &'static str = "true";
const FALSE: &'static str = "false";
const NULL: &'static str = "null";
const IMPORT: &'static str = "import";
const AS: &'static str = "as";
const REF: &'static str = "ref";
const MATCH: &'static str = "match";


named!(p_token_list<&str, Vec<Token> >,
    many0!(
        complete!(p_token)
    )
);

named!(p_token<&str, Token>,
    //ws?
    do_parse!(
        many0!(
            do_parse!(
                alt_complete!(p_comment)        >>
                opt!(alt_complete!(multispace)) >>
                (0)
            )
        )                               >>
        t: alt!(
            p_punctuators   |
            p_operators     |
            p_float_lit     |
            p_int_lit       |
            p_str_lit       |
            p_keywords      |
            p_id
        )                               >>
        opt!(alt_complete!(multispace)) >>
        (t)
    )
);

named!(p_comment<&str, usize>,
    alt!(
        do_parse!(
            tag!("//")          >>
            take_until!("\n")   >>
            tag!("\n")          >>
            (0)
        )   |
        do_parse!(
            tag!("/*")          >>
            take_until!("*/")   >>
            tag!("*/")          >>
            (0)
        )
    )
);

named!(p_int_lit<&str, Token>,
    do_parse!(
        i: digit >>
        (Token::IntLit(str_to_int(i).unwrap()))
    )
);

named!(p_float_lit<&str, Token>,
    do_parse!(
        i: opt!(digit)  >>
        tag!(".")       >>
        peek!(
            not!(tag!("."))
        )               >>
        f: opt!(digit)  >>
        (make_float(i,f).unwrap())
    )
);

named!(p_str_lit<&str, Token>,
    alt!(
        do_parse!(
            tag!("\"")              >>
            s: take_until!("\"")    >>
            tag!("\"")              >>
            (Token::StrLit(s.to_string()))
        )   |
        do_parse!(
            tag!("\'")              >>
            s: take_until!("\'")    >>
            tag!("\'")              >>
            (Token::StrLit(s.to_string()))
        )
    )
);

named!(p_punctuators<&str, Token>,
    alt!(
        value!(Token::LBrac, tag!("{"))         |
        value!(Token::RBrac, tag!("}"))         |
        value!(Token::LPar, tag!("("))          |
        value!(Token::RPar, tag!(")"))          |
        value!(Token::LSq, tag!("["))           |
        value!(Token::RSq, tag!("]"))           |
        value!(Token::Comma, tag!(","))         |
        value!(Token::SemiColon, tag!(";"))     |
        value!(Token::DoubleColon, tag!("::"))  |
        value!(Token::Colon, tag!(":"))         |
        value!(Token::Arrow, tag!("->"))        |
        value!(Token::DoubleDot, tag!(".."))
    )
);

named!(p_operators<&str, Token>,
    alt!(
        value!(Token::AsnPlus, tag!("+="))      |
        value!(Token::AsnMinus, tag!("-="))     |
        value!(Token::AsnTimes, tag!("*="))     |
        value!(Token::AsnDivide, tag!("/="))    |
        value!(Token::AsnModulo, tag!("%="))    |
        value!(Token::AsnOr, tag!("|="))        |
        value!(Token::AsnXor, tag!("^="))       |
        value!(Token::AsnAnd, tag!("&="))       |
        value!(Token::Plus, tag!("+"))          |
        value!(Token::Minus, tag!("-"))         |
        value!(Token::Times, tag!("*"))         |
        value!(Token::Divide, tag!("/"))        |
        value!(Token::Modulo, tag!("%"))        |
        value!(Token::Or, tag!("|"))            |
        value!(Token::Xor, tag!("^"))           |
        value!(Token::And, tag!("&"))           |
        value!(Token::LEq, tag!("<="))          |
        value!(Token::LThan, tag!("<"))         |
        value!(Token::GEq, tag!(">="))          |
        value!(Token::GThan, tag!(">"))         |
        value!(Token::TrueEq, tag!("==="))      |
        value!(Token::Equal, tag!("=="))        |
        value!(Token::TrueNEq, tag!("!=="))     |
        value!(Token::NEqual, tag!("!="))       |
        value!(Token::Not, tag!("!"))           |
        value!(Token::Assign, tag!("="))
    )
);

named!(p_keywords<&str, Token>,
    do_parse!(
        t: alt!(
            value!(Token::Var, tag!(VAR))           |
            value!(Token::Func, tag!(FUNC))         |
            value!(Token::Return, tag!(RETURN))     |
            value!(Token::If, tag!(IF))             |
            value!(Token::Elif, tag!(ELIF))         |
            value!(Token::Else, tag!(ELSE))         |
            value!(Token::While, tag!(WHILE))       |
            value!(Token::For, tag!(FOR))           |
            value!(Token::In, tag!(IN))             |
            value!(Token::Continue, tag!(CONTINUE)) |
            value!(Token::Break, tag!(BREAK))       |
            value!(Token::True, tag!(TRUE))         |
            value!(Token::False, tag!(FALSE))       |
            value!(Token::Null, tag!(NULL))         |
            value!(Token::Import, tag!(IMPORT))     |
            value!(Token::As, tag!(AS))             |
            value!(Token::Ref, tag!(REF))           |
            value!(Token::Match, tag!(MATCH))
        )                           >>
        peek!(not!(alphanumeric))   >>
        (t)
    )
);

named!(p_id<&str, Token>,
    do_parse!(
        peek!(alpha)    >>
        id: take_while!(
            |c: char| c.is_alphanumeric() || (c == '_')
        )               >>
        (Token::Id(id.to_string()))
    )
);

fn str_to_int(s: &str) -> Result<i64, String> {
    match s.parse::<i64>() {
        Ok(i) => Ok(i),
        Err(_) => Err(format!("Not an integer: {}", s)),
    }
}

fn make_float(i: Option<&str>, f: Option<&str>) -> Result<Token, String> {
    //use std::f64;
    match (i,f) {
        (Some(i), Some(f)) => {
            let ten: f64 = 10.0;
            let div = ten.powi(f.len() as i32);
            match (str_to_int(i), str_to_int(f)) {
                (Ok(i), Ok(f)) => Ok(Token::FloatLit((i as f64) + (f as f64) / div)),
                (_,_) => Err(format!("Not a float: {}.{}", i, f)),
            }
        },
        (Some(i), None) => match str_to_int(i) {
            Ok(i) => Ok(Token::FloatLit(i as f64)),
            _ => Err(format!("Not a float: {}.", i)),
        },
        (None, Some(f)) => {
            let ten: f64 = 10.0;
            let div = ten.powi(f.len() as i32);
            match str_to_int(f) {
                Ok(f) => Ok(Token::FloatLit((f as f64) / div)),
                _ => Err(format!("Not a float: .{}", f)),
            }
        },
        (None, None) => Ok(Token::Dot),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::Token;

    #[test]
    fn tokenise_numbers() {
        let input = "1.1, 203, .15, 22., 50;";
        let expect = vec![Token::FloatLit(1.1), Token::Comma,
                          Token::IntLit(203), Token::Comma,
                          Token::FloatLit(0.15), Token::Comma,
                          Token::FloatLit(22.0), Token::Comma,
                          Token::IntLit(50), Token::SemiColon];

        assert_eq!(tokenise(input).unwrap(), expect);
    }

    #[test]
    fn tokenise_function() {
        let input = "func f(x) {return x*2;}";
        let expect = vec![Token::Func, Token::Id("f".to_string()), Token::LPar,
                          Token::Id("x".to_string()), Token::RPar, Token::LBrac,
                          Token::Return, Token::Id("x".to_string()), Token::Times,
                          Token::IntLit(2), Token::SemiColon, Token::RBrac];

        assert_eq!(tokenise(input).unwrap(), expect);
    }

    #[test]
    fn tokenise_rec_function() {
        let input = "func f(x) {if x > 1 {return x * f(x-1);} else {return 1;}}";
        let expect = vec![Token::Func, Token::Id("f".to_string()), Token::LPar,
                          Token::Id("x".to_string()), Token::RPar, Token::LBrac,
                          Token::If, Token::Id("x".to_string()), Token::GThan,
                          Token::IntLit(1), Token::LBrac, Token::Return,
                          Token::Id("x".to_string()), Token::Times, Token::Id("f".to_string()),
                          Token::LPar, Token::Id("x".to_string()), Token::Minus,
                          Token::IntLit(1), Token::RPar, Token::SemiColon, Token::RBrac,
                          Token::Else, Token::LBrac, Token::Return, Token::IntLit(1),
                          Token::SemiColon, Token::RBrac, Token::RBrac];

        assert_eq!(tokenise(input).unwrap(), expect);
    }
}
