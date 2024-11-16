use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use crate::models::search_condition::STARTS_WITH;

use super::Filter;

pub struct StartsWith {
    keyword: String,
}

impl StartsWith {
    pub fn new(keyword: String) -> Arc<Self> {
        Arc::new(Self { keyword })
    }
}

impl Filter for StartsWith {
    fn filter(&self, text: &str) -> bool {
        text.starts_with(&self.keyword)
    }
}

impl Debug for StartsWith {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for StartsWith {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}({})", STARTS_WITH, self.keyword)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("test", "test_string", true)]
    #[case("string", "test_string", false)]
    #[case("Test", "test_string", false)]
    fn test(#[case] keyword: String, #[case] text: String, #[case] expected: bool) {
        assert_eq!(StartsWith::new(keyword).filter(text.as_str()), expected)
    }
}
