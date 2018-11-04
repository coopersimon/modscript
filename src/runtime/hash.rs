use super::{Value, VType};
use error::{Error, Type, RunCode};

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub type HashV = u64;

pub fn hash_value(val: &Value) -> Result<HashV, Error> {
    use self::Value::*;
    use self::VType::*;

    fn hash_vtype<H: Hasher>(val: &VType, state: &mut H) -> Result<(), Error> {
        match *val {
            I(n) => Ok(n.hash(state)),
            B(b) => Ok(b.hash(state)),
            F(_) => Err(Error::new(Type::RunTime(RunCode::ValueNotHashable))),
        }
    }

    fn internal_hash_value<H: Hasher>(val: &Value, state: &mut H) -> Result<(), Error> {
        match *val {
            Val(ref v)  => hash_vtype(&v, state),
            Ref(ref r)  => hash_vtype(&r.borrow(), state),
            Pair(ref l, ref r) => {
                internal_hash_value(&l.borrow(), state)?;
                internal_hash_value(&r.borrow(), state)
            },
            Str(ref s)  => Ok(s.borrow().hash(state)),
            _           => Err(Error::new(Type::RunTime(RunCode::ValueNotHashable))),
        }
    }

    let mut state = DefaultHasher::new();
    match internal_hash_value(val, &mut state) {
        Ok(())  => Ok(state.finish()),
        Err(e)  => Err(e),
    }
}
