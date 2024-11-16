use super::Matcher;
use anyhow::Result;
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct Regex {
    re: regex::Regex,
}

impl Regex {
    pub fn new(regex: String) -> Result<Arc<Self>> {
        let re = regex::Regex::new(&regex)?;
        Ok(Arc::new(Regex { re }))
    }
}

impl Matcher for Regex {
    fn find(&self, text: &str) -> Vec<(String, Range<usize>)> {
        self.re
            .find_iter(text)
            .map(|m| (text[m.start()..m.end()].to_string(), m.range()))
            .collect()
    }
}

impl Debug for Regex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Regex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "regex({})", self.re)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "te.t",
        "test_test_string",
        vec![
            ("test".to_string(), Range{start: 0, end: 4}),
            ("test".to_string(), Range{start: 5, end: 9})
        ]
    )]
    #[case("te.+", "tst_string", Vec::new())]
    #[should_panic]
    #[case("++", "TST_STRING", Vec::new())]
    fn test(
        #[case] regex: String,
        #[case] text: String,
        #[case] expected: Vec<(String, Range<usize>)>,
    ) {
        assert_eq!(Regex::new(regex).unwrap().find(text.as_str()), expected)
    }
}
