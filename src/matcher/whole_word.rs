use super::Matcher;
use regex::Regex;
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct WholeWord {
    keyword: String,
    re: Regex,
}

impl WholeWord {
    pub fn new(keyword: String) -> Arc<Self> {
        let re = Regex::new(&format!(r"\b{}\b", regex::escape(&keyword))).unwrap();
        Arc::new(WholeWord {
            keyword: keyword.to_string(),
            re,
        })
    }
}

impl Matcher for WholeWord {
    fn find(&self, text: &str) -> Vec<(String, Range<usize>)> {
        self.re
            .find_iter(text)
            .map(|m| (self.keyword.clone(), m.range()))
            .collect()
    }
}

impl Debug for WholeWord {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for WholeWord {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "whole_word({})", self.re.to_string().replace("\\b", ""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "test", "test string",
        vec![
            ("test".to_string(), Range{start: 0, end:4})
        ],
    )]
    #[case(
        "string", "test string",
        vec![
            ("string".to_string(), Range{start: 5, end:11})
        ],
    )]
    #[case("test", "testa_string", Vec::new())]
    fn test(
        #[case] keyword: String,
        #[case] text: String,
        #[case] expected: Vec<(String, Range<usize>)>,
    ) {
        assert_eq!(WholeWord::new(keyword).find(text.as_str()), expected)
    }
}
