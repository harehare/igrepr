use super::Filter;
use crate::models::search_condition::ENDS_WITH;
use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

pub struct Contains {
    keyword: String,
}

impl Contains {
    pub fn new(keyword: String) -> Arc<Self> {
        Arc::new(Contains { keyword })
    }
}

impl Filter for Contains {
    fn filter(&self, text: &str) -> bool {
        text.contains(&self.keyword)
    }
}

impl Debug for Contains {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Contains {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}({})", ENDS_WITH, self.keyword)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("test", "test_string", true)]
    #[case("string", "test_string", true)]
    #[case("Test", "test_string", false)]
    fn test(#[case] keyword: String, #[case] text: String, #[case] expected: bool) {
        assert_eq!(Contains::new(keyword).filter(text.as_str()), expected)
    }
}
