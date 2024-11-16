use crate::models::search_condition::CAMEL_CASE;

use super::Transform;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct CamelCase {}

impl CamelCase {
    pub fn new() -> Arc<Self> {
        Arc::new(CamelCase {})
    }
}

impl Transform for CamelCase {
    fn transform(&self, text: &str) -> Vec<(String, Range<usize>)> {
        vec![(
            text.to_case(Case::Camel).to_string(),
            Range {
                start: 0,
                end: text.len(),
            },
        )]
    }
}

impl Debug for CamelCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for CamelCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}()", CAMEL_CASE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("test_string", vec![("testString".to_string(), Range{start: 0, end: 11})])]
    #[case("testString", vec![("testString".to_string(), Range{start: 0, end: 10})])]
    fn test(#[case] text: String, #[case] expected: Vec<(String, Range<usize>)>) {
        assert_eq!(CamelCase::new().transform(text.as_str()), expected)
    }
}
