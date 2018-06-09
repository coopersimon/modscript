// Core type functions
use super::{Value, ExprRes, expr_err};
use std::rc::Rc;
use std::cell::RefCell;

pub fn core_func_call(func: &str, base_type: Value, args: &[Value]) -> ExprRes {
    use Value::*;
    match (base_type, func, args.len()) {
        (Int(i), "to_string",   0) => Ok(Str(Rc::new(RefCell::new(i.to_string())))),
        (Int(i), "to_float",    0) => Ok(Float(i as f64)),
        (Int(i), "abs",         0) => Ok(Int(i.abs())),

        (Float(f), "to_string", 0) => Ok(Str(Rc::new(RefCell::new(f.to_string())))),
        (Float(f), "abs",       0) => Ok(Float(f.abs())),
        (Float(f), "floor",     0) => Ok(Float(f.floor())),
        (Float(f), "ceil",      0) => Ok(Float(f.ceil())),
        (Float(f), "round",     0) => Ok(Float(f.round())),

        //(Str(ref s), "len",     0) => Ok(
        (Str(ref s), "clone",   0) => Ok(Str(Rc::new(RefCell::new(s.borrow().clone())))),
        (Str(ref s), "concat",  1) => match args[0] {
            Str(ref sb) => {s.borrow_mut().push_str(&*sb.borrow()); Ok(Null)},
            _           => expr_err("Concat argument must be str."),
        },
        (Str(ref s), "parse_num", 0) => match s.borrow().parse::<i64>() {
            Ok(i)   => Ok(Int(i)),
            Err(_)  => match s.borrow().parse::<f64>() {
                Ok(f)   => Ok(Float(f)),
                Err(_)  => expr_err("Cannot parse string."),
            },
        },

        (List(ref l), "len",    0) => Ok(Int(l.borrow().len() as i64)),
        (List(ref l), "clone",  0) => Ok(List(Rc::new(RefCell::new(l.borrow().clone())))),
        (List(ref l), "append", 1) => {l.borrow_mut().push(args[0].clone()); Ok(Null)},
        (List(ref l), "concat", 1) => match args[0] {
            List(ref lb) => {l.borrow_mut().extend_from_slice(lb.borrow().as_slice()); Ok(Null)},
            _            => expr_err("'concat' argument must be list."),
        },
        (List(ref l), "front",  0) => match l.borrow().first() {
            Some(v) => Ok(v.clone()),
            None    => expr_err("To access front of list, must have length > 0."),
        },
        (List(ref l), "back",   0) => match l.borrow().last() {
            Some(v) => Ok(v.clone()),
            None    => expr_err("To access back of list, must have length > 0."),
        },

        (Obj(ref o), "clone",   0) => Ok(Obj(Rc::new(RefCell::new(o.borrow().clone())))),
        (Obj(ref o), "is_field",1) => match args[0] {
            Str(ref s) => Ok(Bool(o.borrow().contains_key(&*s.borrow()))),
            _          => expr_err("'is_field' argument must be string."),
        },
        /*(Obj(ref o), "similar", 1) => match args[0] {
            Obj(ref ob) => {
                if o.borrow().len() < ob.borrow().len() {
                    return Ok(Bool(false));
                }
                for (fa,fb) in o.borrow().keys().zip(ob.borrow().keys()) {
                    if fa != fb {
                        return Ok(Bool(false));
                    }
                }
                Ok(Bool(true))
            },
            _           => expr_err("'similar' argument must be object."),
        },*/
        (Obj(ref o), "same",    1) => match args[0] {
            Obj(ref ob) => {
                if o.borrow().len() != ob.borrow().len() {
                    return Ok(Bool(false));
                }
                for (fa,fb) in o.borrow().keys().zip(ob.borrow().keys()) {
                    if fa != fb {
                        return Ok(Bool(false));
                    }
                }
                Ok(Bool(true))
            },
            _           => expr_err("'same' argument must be object."),
        },

        (_,_,_) => expr_err("Invalid core call."),
    }
}
