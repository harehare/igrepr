use std::fmt::{self, Display, Formatter};

use anyhow::{anyhow, Result};

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Str(String),
    Num(usize),
    Env(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Eq(Value),
    Ne(Value),
    Gt(Value),
    Gte(Value),
    Lt(Value),
    Lte(Value),
}

impl Value {
    pub fn int_value(&self) -> Result<usize> {
        match self {
            Value::Num(n) => Ok(*n),
            Value::Env(e) => match std::env::var(e) {
                Ok(v) => match v.parse::<usize>() {
                    Ok(n) => Ok(n),
                    Err(_) => Err(anyhow!("Value is not a number")),
                },
                Err(_) => Err(anyhow!("Environment variable not found")),
            },
            _ => Err(anyhow!("Value is not a number")),
        }
    }

    pub fn string_value(&self) -> Result<String> {
        match self {
            Value::Num(n) => Ok(n.to_string()),
            Value::Env(e) => match std::env::var(e) {
                Ok(v) => Ok(v),
                Err(_) => Err(anyhow!("Environment variable not found")),
            },
            Value::Str(s) => Ok(s.to_string()),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            Value::Str(s) => s.to_string(),
            Value::Num(n) => n.to_string(),
            Value::Env(e) => format!("env.{}", e),
        };
        write!(f, "{}", s)
    }
}
