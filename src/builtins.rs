use crate::interp::*;

macro_rules! get_int {
    ($val:expr) => {
        match $val {
            Value::Int(n) => n,
            _ => panic!("Expected Int, got: {}", $val),
        }
    };
}

macro_rules! get_bool {
    ($val:expr) => {
        match $val {
            Value::Bool(b) => b,
            _ => panic!("Expected Bool, got: {}", $val),
        }
    };
}

pub fn add(args:Vec<Value>) -> Value {
    assert!(args.len() == 2);
    let x = get_int!(args[0]);
    let y = get_int!(args[1]);
    Value::Int(x + y)
}

pub fn sub(args: Vec<Value>) -> Value {
    assert!(args.len() == 2);
    let x = get_int!(args[0]);
    let y = get_int!(args[1]);
    Value::Int(x - y)
}

pub fn mul(args: Vec<Value>) -> Value {
    assert!(args.len() == 2);
    let x = get_int!(args[0]);
    let y = get_int!(args[1]);
    Value::Int(x * y)
}

pub fn div(args: Vec<Value>) -> Value {
    assert!(args.len() == 2);
    let x = get_int!(args[0]);
    let y = get_int!(args[1]);
    Value::Int(x / y)
}

pub fn mod_(args: Vec<Value>) -> Value {
    assert!(args.len() == 2);
    let x = get_int!(args[0]);
    let y = get_int!(args[1]);
    Value::Int(x % y)
}

pub fn bnot(args: Vec<Value>) -> Value {
    assert!(args.len() == 1);
    let x = get_bool!(args[0]);
    Value::Bool(!x)
}

pub fn eq(args: Vec<Value>) -> Value {
    assert!(args.len() == 2);
    let x = get_int!(args[0]);
    let y = get_int!(args[1]);
    Value::Bool(x == y)
}

pub fn neq(args: Vec<Value>) -> Value {
    assert!(args.len() == 2);
    let x = get_int!(args[0]);
    let y = get_int!(args[1]);
    Value::Bool(x != y)
}

pub fn lt(args: Vec<Value>) -> Value {
    assert!(args.len() == 2);
    let x = get_int!(args[0]);
    let y = get_int!(args[1]);
    Value::Bool(x < y)
}

pub fn gt(args: Vec<Value>) -> Value {
    assert!(args.len() == 2);
    let x = get_int!(args[0]);
    let y = get_int!(args[1]);
    Value::Bool(x > y)
}

pub fn le(args: Vec<Value>) -> Value {
    assert!(args.len() == 2);
    let x = get_int!(args[0]);
    let y = get_int!(args[1]);
    Value::Bool(x <= y)
}

pub fn ge(args: Vec<Value>) -> Value {
    assert!(args.len() == 2);
    let x = get_int!(args[0]);
    let y = get_int!(args[1]);
    Value::Bool(x >= y)
}

pub fn and(args: Vec<Value>) -> Value {
    assert!(args.len() == 2);
    let x = get_bool!(args[0]);
    let y = get_bool!(args[1]);
    Value::Bool(x && y)
}

pub fn or(args: Vec<Value>) -> Value {
    assert!(args.len() == 2);
    let x = get_bool!(args[0]);
    let y = get_bool!(args[1]);
    Value::Bool(x || y)
}

pub fn not(args: Vec<Value>) -> Value {
    assert!(args.len() == 1);
    let x = get_bool!(args[0]);
    Value::Bool(!x)
}

pub fn builtin_println(args: Vec<Value>) -> Value {
    assert!(args.len() == 1);
    println!("{}", args[0]);
    Value::Unit
}