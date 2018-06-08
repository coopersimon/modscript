mod resolver;
mod tokeniser;
mod parser;
mod expr;

pub use self::parser::{parse_package, parse_snippet};
pub use self::tokeniser::tokenise;


#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // Variants
    Id(String),
    IntLit(i64),
    FloatLit(f64),
    StrLit(String),
    Null,

    // Keywords
    Var,
    Func,
    Return,
    If,
    Elif,
    Else,
    While,
    True,
    False,
    For,
    In,
    Continue,
    Break,
    Import,
    As,

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
    Colon,
    DoubleColon,
    SemiColon,
    Arrow,
    Dot,
    LSq,
    RSq,
}
