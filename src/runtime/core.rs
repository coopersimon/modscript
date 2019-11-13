// Core type functions
use super::{Value, VType, ExprRes, hash_value};
use error::{mserr, Type, RunCode};
use std::rc::Rc;
use std::cell::RefCell;

pub fn core_func_call(func: &str, base_type: Value, args: &[Value]) -> ExprRes {
    match func {
        "to_string" =>  to_string(base_type, args),
        "to_float"  =>  to_float(base_type, args),
        "abs"       =>  abs(base_type, args),
        "floor"     =>  floor(base_type, args),
        "ceil"      =>  ceil(base_type, args),
        "round"     =>  round(base_type, args),
        "len"       =>  len(base_type, args),
        // left, right OR first, second OR front, back
        "clone"     =>  clone(base_type, args),
        "concat"    =>  concat(base_type, args),
        "parse_num" =>  parse_num(base_type, args),
        "append"    =>  append(base_type, args),
        "pop"       =>  pop(base_type, args),
        "front"     =>  front(base_type, args),
        "back"      =>  back(base_type, args),
        "contains"  =>  contains(base_type, args),
        "is_field"  =>  is_field(base_type, args),
        "same"      =>  same(base_type, args),
        "insert"    =>  insert(base_type, args),
        /*"is_key"    =>  is_key(base_type, args),
        "is_value"  =>  is_value(base_type, args),
        "keys"      =>  keys(base_type, args),
        "values"    =>  values(base_type, args),*/
        _           =>  mserr(Type::RunTime(RunCode::CoreFunctionNotFound)),
    }
}

fn to_string(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;
    use self::VType::*;

    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        Val(I(i))       => Ok(Str(Rc::new(RefCell::new(i.to_string())))),
        Val(F(f))       => Ok(Str(Rc::new(RefCell::new(f.to_string())))),
        Val(B(true))    => Ok(Str(Rc::new(RefCell::new("true".to_string())))),
        Val(B(false))   => Ok(Str(Rc::new(RefCell::new("false".to_string())))),
        Ref(ref r)      => match *r.borrow() {
            I(i)            => Ok(Str(Rc::new(RefCell::new(i.to_string())))),
            F(f)            => Ok(Str(Rc::new(RefCell::new(f.to_string())))),
            B(true)         => Ok(Str(Rc::new(RefCell::new("true".to_string())))),
            B(false)        => Ok(Str(Rc::new(RefCell::new("false".to_string())))),
        },
        _               => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn to_float(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;
    use self::VType::*;

    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        Val(I(i))   => Ok(Val(F(i as f64))),
        Ref(ref r)  => match *r.borrow() {
            I(i)    => Ok(Val(F(i as f64))),
            _       => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn abs(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;
    use self::VType::*;

    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        Val(I(i))   => Ok(Val(I(i.abs()))),
        Val(F(f))   => Ok(Val(F(f.abs()))),
        Ref(ref r)  => match *r.borrow() {
            I(i)    => Ok(Val(I(i.abs()))),
            F(f)    => Ok(Val(F(f.abs()))),
            _       => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn floor(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;
    use self::VType::*;

    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        Val(F(f))   => Ok(Val(I(f.floor() as i64))),
        Ref(ref r)  => match *r.borrow() {
            F(f)    => Ok(Val(I(f.floor() as i64))),
            _       => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn ceil(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;
    use self::VType::*;

    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        Val(F(f))   => Ok(Val(I(f.ceil() as i64))),
        Ref(ref r)  => match *r.borrow() {
            F(f)    => Ok(Val(I(f.ceil() as i64))),
            _       => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn round(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;
    use self::VType::*;

    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        Val(F(f))   => Ok(Val(I(f.round() as i64))),
        Ref(ref r)  => match *r.borrow() {
            F(f)    => Ok(Val(I(f.round() as i64))),
            _       => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn len(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;
    use self::VType::*;

    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        /*Str(ref s) => ,*/
        List(ref l) => Ok(Val(I(l.borrow().len() as i64))),
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn clone(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;
    use self::VType::*;

    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        Val(v)      => Ok(Val(v.clone())),
        Str(ref s)  => Ok(Str(Rc::new(RefCell::new(s.borrow().clone())))),
        List(ref l) => Ok(List(Rc::new(RefCell::new(l.borrow().clone())))),
        Obj(ref o)  => Ok(Obj(Rc::new(RefCell::new(o.borrow().clone())))),
        Map(ref m)  => Ok(Map(Rc::new(RefCell::new(m.borrow().clone())))),
        Ref(ref r)  => match *r.borrow() {
            I(i)    => Ok(Val(I(i))),
            F(f)    => Ok(Val(F(f))),
            B(b)    => Ok(Val(B(b))),
        },
        // TODO: clone everything? (especially closures)
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn concat(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        Str(ref s) => match args[0] {
            Str(ref sb) => {s.borrow_mut().push_str(&*sb.borrow()); Ok(Null)},
            _           => mserr(Type::RunTime(RunCode::CoreArgumentTypeError)),
        },
        List(ref l) => match args[0] {
            List(ref lb) => {l.borrow_mut().extend_from_slice(lb.borrow().as_slice()); Ok(Null)},
            _            => mserr(Type::RunTime(RunCode::CoreArgumentTypeError)),
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn parse_num(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;
    use self::VType::*;

    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        Str(ref s)  => match s.borrow().parse::<i64>() {
            Ok(i)   => Ok(Val(I(i))),
            Err(_)  => match s.borrow().parse::<f64>() {
                Ok(f)   => Ok(Val(F(f))),
                Err(_)  => mserr(Type::RunTime(RunCode::CoreParseError)),
            },
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn append(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        List(ref l) => {l.borrow_mut().push(args[0].clone()); Ok(Null)},
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn pop(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;

    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        Str(ref s)  => {
            s.borrow_mut().pop();
            Ok(Null)
        },
        List(ref l) => {
            l.borrow_mut().pop();
            Ok(Null)
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn front(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;

    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        List(ref l) => match l.borrow().first() {
            Some(v) => Ok(v.clone()),
            None    => mserr(Type::RunTime(RunCode::CoreAccessError)),
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn back(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;

    if args.len() != 0 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        List(ref l) => match l.borrow().last() {
            Some(v) => Ok(v.clone()),
            None    => mserr(Type::RunTime(RunCode::CoreAccessError)),
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn contains(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;
    use self::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        List(ref l) => {
            for i in l.borrow().iter() {
                if args[0] == *i {
                    return Ok(Val(B(true)));
                }
            }
            Ok(Val(B(false)))
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn is_field(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;
    use self::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        Obj(ref o) => match args[0] {
            Str(ref s) => Ok(Val(B(o.borrow().contains_key(&*s.borrow())))),
            _          => mserr(Type::RunTime(RunCode::CoreArgumentTypeError)),
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn same(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;
    use self::VType::*;

    if args.len() != 1 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        Obj(ref o) => match args[0] {
            Obj(ref ob) => {
                if o.borrow().len() != ob.borrow().len() {
                    return Ok(Val(B(false)));
                }
                for (fa,fb) in o.borrow().keys().zip(ob.borrow().keys()) {
                    if fa != fb {
                        return Ok(Val(B(false)));
                    }
                }
                Ok(Val(B(true)))
            },
            _           => mserr(Type::RunTime(RunCode::CoreArgumentTypeError)),
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}

fn insert(base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;

    if args.len() != 2 {
        return mserr(Type::RunTime(RunCode::CoreWrongNumberOfArguments));
    }

    match base_type {
        Map(ref m) => {
            let keyhash = hash_value(&args[0])?;
            let keyclone = clone(args[0].clone(), &[])?;
            m.borrow_mut().insert(keyhash,(keyclone,args[1].clone()));
            Ok(Null)
        },
        _           => mserr(Type::RunTime(RunCode::CoreBaseTypeError)),
    }
}
