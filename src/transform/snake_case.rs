use crate::models::search_condition::SNAKE_CASE;

use super::Transform;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct SnakeCase {}

impl SnakeCase {
    pub fn new() -> Arc<Self> {
        Arc::new(SnakeCase {})
    }
}

impl Transform for SnakeCase {
    fn transform(&self, text: &str) -> Vec<(String, Range<usize>)> {
        vec![(
            text.to_case(Case::Snake).to_string(),
            Range {
                start: 0,
                end: text.len(),
            },
        )]
    }
}

impl Debug for SnakeCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for SnakeCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}()", SNAKE_CASE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("testString", vec![("test_string".to_string(), Range{start: 0, end: 10})])]
    #[case("test_string", vec![("test_string".to_string(), Range{start: 0, end: 11})])]
    fn test(#[case] text: String, #[case] expected: Vec<(String, Range<usize>)>) {
        assert_eq!(SnakeCase::new().transform(text.as_str()), expected)
    }
}
