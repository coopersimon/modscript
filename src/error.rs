use runtime::Value;
use std::fmt;

pub fn mserr(code: Type) -> Result<Value, Error> {
    Err(Error::new(code))
}

pub struct Error {
    err_code: Type,
    // line
    // Function name?
    // Package?
}

impl Error {
    pub fn new(code: Type) -> Self {
        Error {
            err_code: code,
        }
    }

    pub fn to_string(&self) -> String {
        use Type::*;
        match self.err_code {
            Exception(ref v)    => format!("Exception occurred: {}", v),
            CompileTime(ref ec) => format!("Compile error: {:?}", ec),
            RunTime(ref ec)     => format!("Runtime error: {:?}", ec),
            Critical(ref ec)    => format!("Internal error: {:?}", ec),
            Package(ref pe)     => format!("{}", pe.to_string())
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub enum Type {
    Exception(Value),
    CompileTime(CompileCode),
    RunTime(RunCode),
    Critical(CriticalCode),
    Package(Box<CustomError>)
}

#[derive(Debug)]
pub enum CompileCode {
    Error,
    InvalidFile,
    PackageNotFound,
}

#[derive(Debug)]
pub enum RunCode {
    // Runtime Data Errors
    VariableNotDeclared,
    VariableAlreadyDeclared,
    FunctionNotFound,
    PackageNotFound,

    // Statement/Function Errors
    TypeError,
    InvalidCall,
    WrongNumberOfArguments,
    CannotContinue,
    CannotBreak,

    // Expr Errors
    OutOfBounds,
    FieldNotFound,
    DivideByZero,
    InvalidNegative,
    ValueNotHashable,

    // Core Function Errors
    CoreFunctionNotFound,
    CoreBaseTypeError,
    CoreWrongNumberOfArguments,
    CoreArgumentTypeError,
    CoreParseError,
    CoreAccessError,
}

#[derive(Debug)]
pub enum CriticalCode {
    ScopeError,
    NoDefaultPackage,
}

pub trait CustomError {
    fn to_string(&self) -> String;
}
