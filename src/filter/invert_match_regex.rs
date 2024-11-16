use crate::models::search_condition::INVERT_MATCH_REGEX;

use super::Filter;
use anyhow::Result;
use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

pub struct InvertMatchRegex {
    re: regex::Regex,
}

impl InvertMatchRegex {
    pub fn new(regex: String) -> Result<Arc<Self>> {
        let re = regex::Regex::new(regex.as_str())?;
        Ok(Arc::new(InvertMatchRegex { re }))
    }
}

impl Filter for InvertMatchRegex {
    fn filter(&self, text: &str) -> bool {
        !self.re.is_match(text)
    }
}

impl Debug for InvertMatchRegex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for InvertMatchRegex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}({})", INVERT_MATCH_REGEX, self.re)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("te.t", "test_string", false)]
    #[case("te.+", "tst_string", true)]
    #[should_panic]
    #[case("++", "TST_STRING", false)]
    fn test(#[case] regex: String, #[case] text: String, #[case] expected: bool) {
        assert_eq!(
            InvertMatchRegex::new(regex).unwrap().filter(text.as_str()),
            expected
        )
    }
}
