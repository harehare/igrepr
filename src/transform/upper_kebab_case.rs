use crate::models::search_condition::UPPER_KEBAB_CASE;

use super::Transform;
use convert_case::{Case, Casing};
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct UpperKebabCase {}

impl UpperKebabCase {
    pub fn new() -> Arc<Self> {
        Arc::new(UpperKebabCase {})
    }
}

impl Transform for UpperKebabCase {
    fn transform(&self, text: &str) -> Vec<(String, Range<usize>)> {
        vec![(
            text.to_case(Case::UpperKebab).to_string(),
            Range {
                start: 0,
                end: text.len(),
            },
        )]
    }
}

impl Debug for UpperKebabCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for UpperKebabCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}()", UPPER_KEBAB_CASE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("testString", vec![("TEST-STRING".to_string(), Range{start: 0, end: 10})])]
    #[case("test_string", vec![("TEST-STRING".to_string(), Range{start: 0, end: 11})])]
    fn test(#[case] text: String, #[case] expected: Vec<(String, Range<usize>)>) {
        assert_eq!(UpperKebabCase::new().transform(text.as_str()), expected)
    }
}
