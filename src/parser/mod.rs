mod resolver;
mod tokeniser;
mod parser;
mod expr;

use nom::{InputLength, Compare, CompareResult};


#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // Variants
    Id(String),
    IntLit(i64),
    FloatLit(f64),
    StrLit(String),

    // Keywords
    Var,
    Func,
    Return,
    If,
    Else,
    While,
    True,
    False,
    //For,
    //In,
    //Continue,
    //Break,
    //Import,
    //As,

    // Core functions?

    // Operators
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Or,
    Xor,
    And,
    GThan,
    GEq,
    LThan,
    LEq,
    Equal,
    NEqual,
    TrueEq,
    TrueNEq,
    Not,
    
    // Assignment Ops
    AsnPlus,
    AsnMinus,
    AsnTimes,
    AsnDivide,
    AsnModulo,
    AsnOr,
    AsnXor,
    AsnAnd,
    Assign,

    // Punctuators
    LBrac,
    RBrac,
    LPar,
    RPar,
    Comma,
    DoubleColon,
    SemiColon,
    //Arrow,
    //Dot,
    //LSq,
    //RSq,
}

/*impl InputLength for Token {
    fn input_len(&self) -> usize {
        1
    }
}*/

/*impl nom::AsBytes for Token {
    fn as_bytes
}*/

type TokenSlice<'a> = &'a [Token];

//type A = 'a;
//type B = 'b;

/*impl<'a, 'b> Compare<&'b [Token]> for &'a [Token] {
    fn compare(&'a self, t: &'b [Token]) -> CompareResult {
        if t.len() == 0 {
            return CompareResult::Incomplete;
        } else if self == t {
            return CompareResult::Ok;
        } else {
            return CompareResult::Error;
        }
    }
}*/

/*impl Compare<&'a [Token]> for &'b [Token] {
    fn compare(&self, t: &[Token]) -> CompareResult {
        if t.len() == 0 {
            return CompareResult::Incomplete;
        } else if self == t {
            return CompareResult::Ok;
        } else {
            return CompareResult::Error;
        }
    }
}*/
