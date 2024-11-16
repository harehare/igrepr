use crate::models::search_condition::DELETE;

use super::Transform;
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct Delete {
    range: Range<usize>,
}

impl Delete {
    pub fn new(range: Range<usize>) -> Arc<Self> {
        Arc::new(Delete { range })
    }
}

impl Transform for Delete {
    fn transform(&self, text: &str) -> Vec<(String, Range<usize>)> {
        vec![(
            text.chars()
                .take(self.range.start)
                .chain(text.chars().skip(self.range.end))
                .collect(),
            Range {
                start: 0,
                end: text.len(),
            },
        )]
    }
}

impl Debug for Delete {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Delete {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}({}, {})", DELETE, self.range.start, self.range.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("testString", 0, 4, vec![("String".to_string(), Range{start: 0, end: 10})])]
    #[case("test_string", 4, 11, vec![("test".to_string(), Range{start: 0, end: 11})])]
    fn test(
        #[case] text: String,
        #[case] start: usize,
        #[case] end: usize,
        #[case] expected: Vec<(String, Range<usize>)>,
    ) {
        assert_eq!(
            Delete::new(Range { start, end }).transform(text.as_str()),
            expected
        )
    }
}
