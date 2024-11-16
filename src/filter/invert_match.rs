use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use crate::models::search_condition::INVERT_MATCH;

use super::Filter;

pub struct InvertMatch {
    keyword: String,
}

impl InvertMatch {
    pub fn new(keyword: String) -> Arc<Self> {
        Arc::new(InvertMatch { keyword })
    }
}

impl Filter for InvertMatch {
    fn filter(&self, text: &str) -> bool {
        !text.contains(&self.keyword)
    }
}

impl Debug for InvertMatch {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for InvertMatch {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}({})", INVERT_MATCH, self.keyword)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("test", "test_string", false)]
    #[case("string", "test_string", false)]
    #[case("Test", "test_string", true)]
    fn test(#[case] keyword: String, #[case] text: String, #[case] expected: bool) {
        assert_eq!(InvertMatch::new(keyword).filter(text.as_str()), expected)
    }
}
