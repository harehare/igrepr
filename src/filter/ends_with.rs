use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use crate::models::search_condition::ENDS_WITH;

use super::Filter;

pub struct EndsWith {
    keyword: String,
}

impl EndsWith {
    pub fn new(keyword: String) -> Arc<Self> {
        Arc::new(EndsWith { keyword })
    }
}

impl Filter for EndsWith {
    fn filter(&self, text: &str) -> bool {
        text.ends_with(&self.keyword)
    }
}

impl Debug for EndsWith {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for EndsWith {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}({})", ENDS_WITH, self.keyword)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("test", "test_string", false)]
    #[case("string", "test_string", true)]
    #[case("Test", "test_string", false)]
    fn test(#[case] keyword: String, #[case] text: String, #[case] expected: bool) {
        assert_eq!(EndsWith::new(keyword).filter(text.as_str()), expected)
    }
}
