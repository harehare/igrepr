use super::Matcher;
use crate::models::value::Op;
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct Number {
    re: regex::Regex,
    op: Op,
}

impl Number {
    pub fn new(op: Op) -> Arc<Self> {
        let re = regex::Regex::new("[0-9]+").unwrap();
        Arc::new(Number { re, op })
    }
}

impl Matcher for Number {
    fn find(&self, text: &str) -> Vec<(String, Range<usize>)> {
        self.re
            .find_iter(text)
            .filter_map(|m| {
                let v = text[m.start()..m.end()].to_string();

                v.parse::<usize>().ok().and_then(|n| {
                    match self.op {
                        Op::Eq(ref e) => {
                            if n != e.int_value().unwrap_or_default() {
                                return None;
                            }
                        }
                        Op::Ne(ref e) => {
                            if n == e.int_value().unwrap_or_default() {
                                return None;
                            }
                        }
                        Op::Gt(ref e) => {
                            if n <= e.int_value().unwrap_or_default() {
                                return None;
                            }
                        }
                        Op::Gte(ref e) => {
                            if n < e.int_value().unwrap_or_default() {
                                return None;
                            }
                        }
                        Op::Lt(ref e) => {
                            if n >= e.int_value().unwrap_or_default() {
                                return None;
                            }
                        }
                        Op::Lte(ref e) => {
                            if n > e.int_value().unwrap_or_default() {
                                return None;
                            }
                        }
                    }

                    Some((v, m.range()))
                })
            })
            .collect()
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.op {
            Op::Eq(ref e) => {
                write!(f, "number() == {}", e.int_value().unwrap_or_default())
            }
            Op::Ne(ref e) => {
                write!(f, "number() != {}", e.int_value().unwrap_or_default())
            }
            Op::Gt(ref e) => {
                write!(f, "number() > {}", e.int_value().unwrap_or_default())
            }
            Op::Gte(ref e) => {
                write!(f, "number() >= {}", e.int_value().unwrap_or_default())
            }
            Op::Lt(ref e) => {
                write!(f, "number() < {}", e.int_value().unwrap_or_default())
            }
            Op::Lte(ref e) => {
                write!(f, "number() <= {}", e.int_value().unwrap_or_default())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Value;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "test1234",
        Op::Eq(Value::Num(1234)),
        vec![
            ("1234".to_string(), Range{start: 4, end:8})
        ]
    )]
    #[case(
        "t1234est",
        Op::Eq(Value::Num(5)),
        vec![]
    )]
    #[case(
        "5test",
        Op::Ne(Value::Num(1234)),
        vec![
            ("5".to_string(), Range{start: 0, end:1})
        ]
    )]
    #[case(
        "1234test",
        Op::Ne(Value::Num(1234)),
        vec![]
    )]
    #[case(
        "1test",
        Op::Gt(Value::Num(0)),
        vec![
            ("1".to_string(), Range{start: 0, end:1})
        ]
    )]
    #[case(
        "1test",
        Op::Gt(Value::Num(1)),
        vec![]
    )]
    #[case(
        "1test",
        Op::Gte(Value::Num(1)),
        vec![
            ("1".to_string(), Range{start: 0, end:1})
        ]
    )]
    #[case(
        "0test",
        Op::Gte(Value::Num(1)),
        vec![]
    )]
    #[case(
        "0test",
        Op::Lt(Value::Num(1)),
        vec![
            ("0".to_string(), Range{start: 0, end:1})
        ]
    )]
    #[case(
        "1test",
        Op::Lt(Value::Num(1)),
        vec![]
    )]
    #[case(
        "1test",
        Op::Lte(Value::Num(1)),
        vec![
            ("1".to_string(), Range{start: 0, end:1})
        ]
    )]
    #[case(
        "2test",
        Op::Lte(Value::Num(1)),
        vec![]
    )]
    fn test(#[case] text: String, #[case] op: Op, #[case] expected: Vec<(String, Range<usize>)>) {
        assert_eq!(Number::new(op).find(text.as_str()), expected)
    }
}
