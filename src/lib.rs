#[macro_use]
extern crate nom;

mod ast;
mod runtime;
mod parser;
mod error;

pub use ast::{ScriptPackage, Script, ScriptExpr};
pub use runtime::{Value, VType, Signal, ExprRes, FuncMap, Scope, Callable, PackageRoot};
pub use error::*;
use parser::{tokenise, parse_package, parse_snippet, parse_expr_snippet, Token};

use std::fs::File;
use std::io::{BufReader, Read};

pub fn package_from_file(file_name: &str) -> Result<ScriptPackage, Error> {
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => return Err(Error::new(Type::CompileTime(CompileCode::InvalidFile))),
    };

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).unwrap();

    let tokens = tokenise(&contents)?;

    parse_package(&tokens, file_name)
}


pub fn script_from_text(imports: &[(String,String)], script: &str) -> Result<Script, Error> {
    let tokens = tokenise(script)?;

    parse_snippet(&tokens, imports)
}

pub fn expr_from_text(imports: &[(String,String)], script: &str) -> Result<ScriptExpr, Error> {
    let mut tokens = tokenise(script)?;
    tokens.push(Token::SemiColon);

    parse_expr_snippet(&tokens, imports)
}
