use crate::models::search_condition::IGNORE_CASE;

use super::Matcher;
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct IgnoreCase {
    keyword: String,
}

impl IgnoreCase {
    pub fn new(keyword: String) -> Arc<Self> {
        Arc::new(IgnoreCase { keyword })
    }
}

impl Matcher for IgnoreCase {
    fn find(&self, text: &str) -> Vec<(String, Range<usize>)> {
        text.to_lowercase()
            .match_indices(&self.keyword.to_lowercase())
            .map(|(i, _)| {
                (
                    self.keyword.clone(),
                    Range {
                        start: i,
                        end: i + self.keyword.len(),
                    },
                )
            })
            .collect()
    }
}

impl Debug for IgnoreCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for IgnoreCase {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}({})", IGNORE_CASE, self.keyword)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "test", "test_TEST_string",
        vec![
            ("test".to_string(), Range{start: 0, end:4}),
            ("test".to_string(), Range{start: 5, end:9})
        ]
    )]
    #[case(
        "string", "test_string",
        vec![
            ("string".to_string(), Range{start: 5, end:11})
        ]
    )]
    #[case(
        "TEST", "test_string",
        vec![
            ("TEST".to_string(), Range{start: 0, end:4})
        ]
    )]
    #[case("TEt", "test_string", Vec::new())]
    fn test(
        #[case] keyword: String,
        #[case] text: String,
        #[case] expected: Vec<(String, Range<usize>)>,
    ) {
        assert_eq!(IgnoreCase::new(keyword).find(text.as_str()), expected)
    }
}
