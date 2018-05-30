use super::Token;

use nom::{multispace, alphanumeric, is_alphanumeric, alpha, double_s, is_digit, digit};
use nom::IResult;

pub fn tokenise(input: &str) -> Result<Vec<Token>, String> {
    match p_token_list(input) {
        Ok((_,o)) => Ok(o),
        Err(e) => Err(format!("Error: {:?}", e)),
    }
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


named!(p_token_list<&str, Vec<Token> >,
    many0!(
        complete!(p_token)
    )
);

named!(p_token<&str, Token>,
    //ws?
    do_parse!(
        t: alt!(
            p_punctuators   |
            p_operators     |
            //p_float_lit     |
            p_int_lit       |
            p_str_lit       |
            p_keywords      |
            p_id
        )                               >>
        opt!(alt_complete!(multispace)) >>
        (t)
    )
);

named!(p_int_lit<&str, Token>,
    do_parse!(
        i: digit >>
        (Token::IntLit(str_to_int(i).unwrap()))
    )
);

named!(p_signed_int<&str, &str>,
    recognize!(preceded!(
        opt!(tag!("-")),
        take_while1!(|c: char| c.is_digit(10))
    ))
);

named!(p_float_lit<&str, Token>,
    do_parse!(
        f: double_s >>
        (Token::FloatLit(f))
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
        value!(Token::Comma, tag!(","))         |
        value!(Token::SemiColon, tag!(";"))     |
        value!(Token::DoubleColon, tag!("::"))
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
            value!(Token::Var, tag!(VAR))       |
            value!(Token::Func, tag!(FUNC))     |
            value!(Token::Return, tag!(RETURN)) |
            value!(Token::If, tag!(IF))         |
            value!(Token::Else, tag!(ELSE))     |
            value!(Token::While, tag!(WHILE))   |
            value!(Token::True, tag!(TRUE))     |
            value!(Token::False, tag!(FALSE))
        )                           >>
        peek!(not!(alphanumeric))   >>
        (t)
    )
);

named!(p_id<&str, Token>,
    do_parse!(
        peek!(alpha)    >>
        id: take_while!(
            |c: char| c.is_alphabetic() || (c == '_')
        )               >>
        (Token::Id(id.to_string()))
    )
);

/*fn str_to_int(s: &[u8]) -> Result<i64, String> {
    use std::str;
    match str::from_utf8(s) {
        Ok(i_str) => match i_str.parse::<i64>() {
            Ok(i) => Ok(i),
            Err(_) => Err(format!("Not an integer: {}", i_str)),
        },
        Err(_) => Err(format!("Incorrectly parsed input string.")),
    }
}*/

fn str_to_int(s: &str) -> Result<i64, String> {
    match s.parse::<i64>() {
        Ok(i) => Ok(i),
        Err(_) => Err(format!("Not an integer: {}", s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser::Token;

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
