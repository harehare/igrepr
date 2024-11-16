use crate::models::search_condition::UPPER_SNAKE_CASE;

use super::Transform;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct UpperSnakeCase {}

impl UpperSnakeCase {
    pub fn new() -> Arc<Self> {
        Arc::new(UpperSnakeCase {})
    }
}

impl Transform for UpperSnakeCase {
    fn transform(&self, text: &str) -> Vec<(String, Range<usize>)> {
        vec![(
            text.to_case(Case::UpperSnake).to_string(),
            Range {
                start: 0,
                end: text.len(),
            },
        )]
    }
}

impl Debug for UpperSnakeCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for UpperSnakeCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}()", UPPER_SNAKE_CASE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("testString", vec![("TEST_STRING".to_string(), Range{start: 0, end: 10})])]
    #[case("test_string", vec![("TEST_STRING".to_string(), Range{start: 0, end: 11})])]
    fn test(#[case] text: String, #[case] expected: Vec<(String, Range<usize>)>) {
        assert_eq!(UpperSnakeCase::new().transform(text.as_str()), expected)
    }
}
