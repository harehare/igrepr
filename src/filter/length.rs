use crate::models::value::Op;

use super::Filter;
use std::fmt::{Debug, Display};
use std::sync::Arc;

pub struct Length {
    is_byte: bool,
    op: Op,
}

impl Length {
    pub fn new(is_byte: bool, op: Op) -> Arc<Self> {
        Arc::new(Length { is_byte, op })
    }
}

impl Filter for Length {
    fn filter(&self, text: &str) -> bool {
        let n = if self.is_byte {
            text.len()
        } else {
            text.chars().count()
        };

        match self.op {
            Op::Eq(ref e) => {
                if n == e.int_value().unwrap_or_default() {
                    return true;
                }
            }
            Op::Ne(ref e) => {
                if n != e.int_value().unwrap_or_default() {
                    return true;
                }
            }
            Op::Gt(ref e) => {
                if n > e.int_value().unwrap_or_default() {
                    return true;
                }
            }
            Op::Gte(ref e) => {
                if n >= e.int_value().unwrap_or_default() {
                    return true;
                }
            }
            Op::Lt(ref e) => {
                if n < e.int_value().unwrap_or_default() {
                    return true;
                }
            }
            Op::Lte(ref e) => {
                if n <= e.int_value().unwrap_or_default() {
                    return true;
                }
            }
        }

        false
    }
}

impl Debug for Length {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.op {
            Op::Eq(ref e) => {
                write!(f, "length() == {}", e)
            }
            Op::Ne(ref e) => {
                write!(f, "length() != {}", e)
            }
            Op::Gt(ref e) => {
                write!(f, "length() > {}", e)
            }
            Op::Gte(ref e) => {
                write!(f, "length() >= {}", e)
            }
            Op::Lt(ref e) => {
                write!(f, "length() < {}", e)
            }
            Op::Lte(ref e) => {
                write!(f, "length() <= {}", e)
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
    #[case("123456", false, Op::Gt(Value::Num(5)), true)]
    #[case("12345", false, Op::Gt(Value::Num(5)), false)]
    #[case("12345", false, Op::Gte(Value::Num(5)), true)]
    #[case("1234", false, Op::Gte(Value::Num(5)), false)]
    #[case("1234", false, Op::Lt(Value::Num(5)), true)]
    #[case("12345", false, Op::Lt(Value::Num(5)), false)]
    #[case("12345", false, Op::Lte(Value::Num(5)), true)]
    #[case("123456", false, Op::Lte(Value::Num(5)), false)]
    #[case("12345", false, Op::Eq(Value::Num(5)), true)]
    #[case("1234", false, Op::Eq(Value::Num(5)), false)]
    #[case("1234", false, Op::Ne(Value::Num(5)), true)]
    #[case("12345", false, Op::Ne(Value::Num(5)), false)]
    #[case("あ", false, Op::Eq(Value::Num(1)), true)]
    fn test(#[case] text: String, #[case] is_byte: bool, #[case] op: Op, #[case] expected: bool) {
        assert_eq!(Length::new(is_byte, op).filter(text.as_str()), expected)
    }
}
