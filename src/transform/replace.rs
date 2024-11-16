use crate::models::search_condition::REPLACE;

use super::Transform;
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct Replace {
    from: String,
    to: String,
}

impl Replace {
    pub fn new(from: String, to: String) -> Arc<Self> {
        Arc::new(Replace { from, to })
    }
}

impl Transform for Replace {
    fn transform(&self, text: &str) -> Vec<(String, Range<usize>)> {
        vec![(
            text.replace(&self.from, &self.to),
            Range {
                start: 0,
                end: text.len(),
            },
        )]
    }
}

impl Debug for Replace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Replace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}({}, {})", REPLACE, self.from, self.to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("test_string", "test", "T", vec![("T_string".to_string(), Range{start: 0, end: 11})])]
    #[case("test_string", "string", "", vec![("test_".to_string(), Range{start: 0, end: 11})])]
    fn test(
        #[case] text: String,
        #[case] from: String,
        #[case] to: String,
        #[case] expected: Vec<(String, Range<usize>)>,
    ) {
        assert_eq!(Replace::new(from, to).transform(text.as_str()), expected)
    }
}
