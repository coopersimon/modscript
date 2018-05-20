#[macro_use]
extern crate nom;

mod ast;
mod runtime;
mod parser;

pub use ast::{ScriptPackage, Script};
pub use runtime::{Value, Signal, ExprRes, FuncMap};
use parser::{parse_package, parse_snippet};

use std::fs::File;
use std::io::{BufReader, Read};

pub fn package_from_file(file_name: &str) -> Result<ScriptPackage, String> {
    let file = match File::open(file_name) {
        Ok(f) => f,
        Err(_) => return Err("Couldn't read file.".to_string()),
    };

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents);

    parse_package(file_name, &contents)
}


pub fn script_from_text(imports: &[(String,String)], script: &str) -> Result<Script, String> {
    parse_snippet(imports, script)
}