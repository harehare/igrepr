use crate::models::search_condition::KEBAB_CASE;

use super::Transform;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct KebabCase {}

impl KebabCase {
    pub fn new() -> Arc<Self> {
        Arc::new(KebabCase {})
    }
}

impl Transform for KebabCase {
    fn transform(&self, text: &str) -> Vec<(String, Range<usize>)> {
        vec![(
            text.to_case(Case::Kebab).to_string(),
            Range {
                start: 0,
                end: text.len(),
            },
        )]
    }
}

impl Debug for KebabCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for KebabCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}()", KEBAB_CASE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("testString", vec![("test-string".to_string(), Range{start: 0, end: 10})])]
    #[case("test_string", vec![("test-string".to_string(), Range{start: 0, end: 11})])]
    fn test(#[case] text: String, #[case] expected: Vec<(String, Range<usize>)>) {
        assert_eq!(KebabCase::new().transform(text.as_str()), expected)
    }
}
