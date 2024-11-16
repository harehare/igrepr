use crate::models::search_condition::TRIM;

use super::Transform;
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct Trim {}

impl Trim {
    pub fn new() -> Arc<Self> {
        Arc::new(Trim {})
    }
}

impl Transform for Trim {
    fn transform(&self, text: &str) -> Vec<(String, Range<usize>)> {
        vec![(
            text.trim().to_string(),
            Range {
                start: 0,
                end: text.len(),
            },
        )]
    }
}

impl Debug for Trim {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Trim {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}()", TRIM)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(" test_string", vec![("test_string".to_string(), Range{start: 0, end: 12})])]
    #[case("test_string ", vec![("test_string".to_string(), Range{start: 0, end: 12})])]
    fn test(#[case] text: String, #[case] expected: Vec<(String, Range<usize>)>) {
        assert_eq!(Trim::new().transform(text.as_str()), expected)
    }
}
