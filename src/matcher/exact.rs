use super::Matcher;
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct Exact {
    keyword: String,
}

impl Exact {
    pub fn new(keyword: String) -> Arc<Self> {
        Arc::new(Exact { keyword })
    }
}

impl Matcher for Exact {
    fn find(&self, text: &str) -> Vec<(String, Range<usize>)> {
        text.match_indices(&self.keyword)
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

impl Debug for Exact {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Exact {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.keyword)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "test", "test_test_string",
        vec![
            ("test".to_string(), Range{start: 0, end: 4}),
            ("test".to_string(), Range{start: 5, end: 9})
        ]
    )]
    #[case(
        "t", "test_string",
        vec![
            ("t".to_string() , Range{start: 0, end: 1}),
            ("t".to_string() , Range{start: 3, end: 4}),
            ("t".to_string() , Range{start: 6, end: 7}),
        ]
    )]
    #[case("Test", "test_string", Vec::new())]
    fn test(
        #[case] keyword: String,
        #[case] text: String,
        #[case] expected: Vec<(String, Range<usize>)>,
    ) {
        assert_eq!(Exact::new(keyword).find(text.as_str()), expected)
    }
}
