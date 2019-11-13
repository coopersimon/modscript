use super::{Value, ExprRes};
use error::{mserr, Type, RunCode};
use std::collections::HashMap;

pub type PackageRoot = Box<dyn Fn(&str, &[Value], &FuncMap) -> ExprRes>;

pub struct FuncMap {
    packages: HashMap<String, PackageRoot>,
}

impl FuncMap {
    pub fn new() -> Self {
        FuncMap {
            packages: HashMap::new(),
        }
    }

    pub fn attach_package(&mut self, package_name: &str, package: PackageRoot) {
        self.packages.insert(package_name.to_string(), package);
    }

    pub fn call_fn(&self, package: &str, name: &str, args: &[Value]) -> ExprRes {
        match self.packages.get(package) {
            Some(p) => p(name, args, self),
            None => mserr(Type::RunTime(RunCode::PackageNotFound)),
        }
    }
}
