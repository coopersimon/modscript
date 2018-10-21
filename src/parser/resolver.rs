use error::{Error, Type, CompileCode, CriticalCode};
use std::collections::HashMap;

// used by compiler to resolve references & ids
pub struct Resolver {
    current_package: Option<String>,
    package_refs: HashMap<String, String>,
    local_refs: HashMap<String, String>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            current_package: None,
            package_refs: HashMap::new(),
            local_refs: HashMap::new(),
        }
    }

    pub fn set_package(&mut self, package_name: &str) {
        self.current_package = Some(package_name.to_string());
    }

    pub fn add_package_ref(&mut self, package_ref: &str, package_name: &str) {
        self.package_refs.insert(package_ref.to_string(), package_name.to_string());
    }

    pub fn get_package_ref(&self, package_ref: Option<&str>) -> Result<String,Error> {
        match package_ref {
            Some(s) => match self.local_refs.get(s) {
                Some(s) => Ok(s.clone()),
                None => match self.package_refs.get(s) {
                    Some(s) => Ok(s.clone()),
                    None => Err(Error::new(Type::CompileTime(CompileCode::PackageNotFound))),
                    //None => Err(format!("Couldn't find package \'{}\'.", s)),
                },
            },
            None => match self.current_package {
                Some(ref s) => Ok(s.clone()),
                None => Err(Error::new(Type::Critical(CriticalCode::NoDefaultPackage))),
            },
        }
    }

    pub fn reset_package_refs(&mut self) {
        self.current_package = None;
        self.package_refs.clear();
    }

    pub fn add_local_ref(&mut self, local_ref: &str, package_name: &str) {
        self.local_refs.insert(local_ref.to_string(), package_name.to_string());
    }

    pub fn clear_local_refs(&mut self) {
        self.local_refs.clear();
    }
}
