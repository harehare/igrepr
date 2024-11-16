use crate::models::search_condition::LOWER_CASE;

use super::Transform;
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct LowerCase {}

impl LowerCase {
    pub fn new() -> Arc<Self> {
        Arc::new(LowerCase {})
    }
}

impl Transform for LowerCase {
    fn transform(&self, text: &str) -> Vec<(String, Range<usize>)> {
        vec![(
            text.to_lowercase(),
            Range {
                start: 0,
                end: text.len(),
            },
        )]
    }
}

impl Debug for LowerCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for LowerCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}()", LOWER_CASE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("TEST_STRING", vec![("test_string".to_string(), Range{start: 0, end: 11})])]
    #[case("TESTSTRING", vec![("teststring".to_string(), Range{start: 0, end: 10})])]
    fn test(#[case] text: String, #[case] expected: Vec<(String, Range<usize>)>) {
        assert_eq!(LowerCase::new().transform(text.as_str()), expected)
    }
}
