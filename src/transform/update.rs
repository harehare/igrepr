use super::Transform;
use std::fmt::{Debug, Display};
use std::ops::Range;
use std::sync::Arc;

pub struct Update {
    value: String,
}

impl Update {
    pub fn new(value: String) -> Arc<Self> {
        Arc::new(Update { value })
    }
}

impl Transform for Update {
    fn transform(&self, text: &str) -> Vec<(String, Range<usize>)> {
        vec![(
            self.value.to_owned(),
            Range {
                start: 0,
                end: text.len(),
            },
        )]
    }
}

impl Debug for Update {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Update {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "insert({})", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("testString", "update", vec![("update".to_string(), Range{start: 0, end: 10})])]
    #[case("test", "update", vec![("update".to_string(), Range{start: 0, end: 4})])]
    fn test(
        #[case] text: String,
        #[case] value: String,
        #[case] expected: Vec<(String, Range<usize>)>,
    ) {
        assert_eq!(Update::new(value).transform(text.as_str()), expected)
    }
}
