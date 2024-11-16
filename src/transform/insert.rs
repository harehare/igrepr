use crate::models::search_condition::INSERT;

use super::Transform;
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct Insert {
    index: usize,
    value: String,
}

impl Insert {
    pub fn new(index: usize, value: String) -> Arc<Self> {
        Arc::new(Insert { index, value })
    }
}

impl Transform for Insert {
    fn transform(&self, text: &str) -> Vec<(String, Range<usize>)> {
        let mut updated_text = text.to_string();
        updated_text.insert_str(self.index, self.value.as_ref());

        vec![(
            updated_text.to_string(),
            Range {
                start: 0,
                end: text.len(),
            },
        )]
    }
}

impl Debug for Insert {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Insert {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}({}, {})", INSERT, self.index, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("testString", 0, "$", vec![("$testString".to_string(), Range{start: 0, end: 10})])]
    #[case("test_string", 4, "T", vec![("testT_string".to_string(), Range{start: 0, end: 11})])]
    fn test(
        #[case] text: String,
        #[case] index: usize,
        #[case] value: String,
        #[case] expected: Vec<(String, Range<usize>)>,
    ) {
        assert_eq!(Insert::new(index, value).transform(text.as_str()), expected)
    }
}
