use crate::models::search_condition::REGEX;

use super::Filter;
use anyhow::Result;
use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

pub struct Regex {
    re: regex::Regex,
}

impl Regex {
    pub fn new(regex: String) -> Result<Arc<Self>> {
        let re = regex::Regex::new(regex.as_str())?;
        Ok(Arc::new(Regex { re }))
    }
}

impl Filter for Regex {
    fn filter(&self, text: &str) -> bool {
        self.re.is_match(text)
    }
}

impl Debug for Regex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Regex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}({})", REGEX, self.re)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("te.t", "test_string", true)]
    #[case("te.+", "tst_string", false)]
    #[should_panic]
    #[case("++", "TST_STRING", false)]
    fn test(#[case] regex: String, #[case] text: String, #[case] expected: bool) {
        assert_eq!(Regex::new(regex).unwrap().filter(text.as_str()), expected)
    }
}
