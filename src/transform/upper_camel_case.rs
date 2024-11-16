use crate::models::search_condition::UPPER_CAMEL_CASE;

use super::Transform;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct UpperCamelCase {}

impl UpperCamelCase {
    pub fn new() -> Arc<Self> {
        Arc::new(UpperCamelCase {})
    }
}

impl Transform for UpperCamelCase {
    fn transform(&self, text: &str) -> Vec<(String, Range<usize>)> {
        vec![(
            text.to_case(Case::UpperCamel).to_string(),
            Range {
                start: 0,
                end: text.len(),
            },
        )]
    }
}

impl Debug for UpperCamelCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for UpperCamelCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}()", UPPER_CAMEL_CASE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("test_string", vec![("TestString".to_string(), Range{start: 0, end: 11})])]
    #[case("testString", vec![("TestString".to_string(), Range{start: 0, end: 10})])]
    fn test(#[case] text: String, #[case] expected: Vec<(String, Range<usize>)>) {
        assert_eq!(UpperCamelCase::new().transform(text.as_str()), expected)
    }
}
