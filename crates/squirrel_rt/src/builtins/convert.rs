/// Imports
use crate::{
    builtins::utils,
    refs::{EnvRef, Ref},
    rt::{
        env::Environment,
        value::{Callable, Native, Value},
    },
};
use std::{cell::RefCell, rc::Rc};

/// Any -> int
fn int() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(i) => Value::Int(*i),
            Value::Float(f) => Value::Float(*f),
            other => utils::error(span, &format!("could not convert `{other}` into int value")),
        }),
    })
}

/// Any -> float
fn float() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(i) => Value::Float(*i as f64),
            Value::Float(f) => Value::Float(*f),
            other => utils::error(
                span,
                &format!("could not convert `{other}` into float value"),
            ),
        }),
    })
}

/// Any -> bool
fn bool() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Bool(b) => Value::Bool(*b),
            Value::String(s) if s == "true" => Value::Bool(true),
            Value::String(s) if s == "false" => Value::Bool(false),
            other => utils::error(
                span,
                &format!("could not convert `{other}` into float value"),
            ),
        }),
    })
}

/// Any -> string
fn string() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| Value::String(format!("{}", values.first().unwrap()))),
    })
}

/// Provides `convert` module env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();

    env.force_define("int", Value::Callable(Callable::Native(int())));
    env.force_define("float", Value::Callable(Callable::Native(float())));
    env.force_define("bool", Value::Callable(Callable::Native(bool())));
    env.force_define("string", Value::Callable(Callable::Native(string())));

    Rc::new(RefCell::new(env))
}
