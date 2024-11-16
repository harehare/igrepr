use crate::{filter::Filter, matcher::Matcher, transform::Transform};
use std::fmt;
use std::fmt::Display;
use std::ops::Range;
use std::sync::Arc;

#[derive(Clone, Debug, Eq)]
pub enum MatchResult {
    Found(String, Range<usize>, usize),
    Filtered(String, Range<usize>, usize),
    Transformed(String, Range<usize>, usize),
}

impl Display for MatchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Found(text, r, _) => write!(f, "found_{}_{}_{}", text, r.start, r.end),
            Self::Filtered(text, r, _) => write!(f, "filtered_{}_{}_{}", text, r.start, r.end),
            Self::Transformed(text, r, _) => {
                write!(f, "transformed_{}_{}_{}", text, r.start, r.end)
            }
        }
    }
}

impl PartialEq for MatchResult {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl PartialOrd for MatchResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.range().start.partial_cmp(&other.range().start) {
            Some(std::cmp::Ordering::Equal) => self.index().partial_cmp(&other.index()),
            other => other,
        }
    }
}

impl MatchResult {
    pub fn is_found(&self) -> bool {
        matches!(self, MatchResult::Found(_, _, _))
    }

    pub fn is_transformed(&self) -> bool {
        matches!(self, MatchResult::Transformed(_, _, _))
    }

    pub fn index(&self) -> usize {
        match self {
            Self::Found(_, _, i) => *i,
            Self::Filtered(_, _, i) => *i,
            Self::Transformed(_, _, i) => *i,
        }
    }

    pub fn range(&self) -> Range<usize> {
        match self {
            Self::Found(_, range, _) => range.clone(),
            Self::Filtered(_, range, _) => range.clone(),
            Self::Transformed(_, range, _) => range.clone(),
        }
    }

    pub fn column(&self) -> usize {
        match self {
            Self::Found(_, range, _) => range.start,
            Self::Filtered(_, range, _) => range.start,
            Self::Transformed(_, range, _) => range.start,
        }
    }

    pub fn find(text: String, f: Arc<dyn Matcher>, index: usize) -> Vec<MatchResult> {
        f.find(&text)
            .into_iter()
            .map(|(t, r)| Self::Found(t.to_string(), r, index))
            .collect()
    }

    pub fn filter(&self, filter: Arc<dyn Filter>, index: usize) -> Option<MatchResult> {
        match &self {
            Self::Found(text, range, _) => {
                if filter.filter(text) {
                    Some(Self::Filtered(text.to_string(), range.clone(), index))
                } else {
                    None
                }
            }
            Self::Filtered(text, range, _) => {
                if filter.filter(text) {
                    Some(Self::Filtered(text.to_string(), range.clone(), index))
                } else {
                    None
                }
            }
            Self::Transformed(text, range, _) => {
                if filter.filter(text) {
                    Some(Self::Filtered(text.to_string(), range.clone(), index))
                } else {
                    None
                }
            }
        }
    }

    pub fn transform(&self, transform: Arc<dyn Transform>, index: usize) -> Vec<MatchResult> {
        match &self {
            Self::Found(text, range, _)
            | Self::Filtered(text, range, _)
            | Self::Transformed(text, range, _) => transform
                .transform(text)
                .into_iter()
                .map(|(v, r)| {
                    Self::Transformed(
                        v.to_string(),
                        Range {
                            start: range.start + r.start,
                            end: range.start + r.end,
                        },
                        index,
                    )
                })
                .collect(),
        }
    }

    pub fn apply(&self, text: String) -> String {
        match &self {
            Self::Transformed(token, range, _) => {
                format!("{}{}{}", &text[..range.start], token, &text[range.end..])
            }
            _ => text.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::vec;

    #[rstest]
    #[case(vec![MatchResult::Transformed("transform".to_string(), Range{start: 0, end: 4}, 1)], "test_string".to_string(), "transform_string".to_string())]
    #[case(vec![MatchResult::Transformed("transform".to_string(), Range{start: 0, end: 4}, 1),
                MatchResult::Transformed("test".to_string(), Range{start: 0, end: 4}, 1)], "test_string".to_string(), "testsform_string".to_string())]
    #[case(vec![MatchResult::Found("trqansform".to_string(), Range{start: 0, end: 4}, 1)], "test_string".to_string(), "test_string".to_string())]
    #[case(vec![MatchResult::Filtered("trqansform".to_string(), Range{start: 0, end: 4}, 1)], "test_string".to_string(), "test_string".to_string())]
    fn apply_match_test(
        #[case] m: Vec<MatchResult>,
        #[case] text: String,
        #[case] expected: String,
    ) {
        assert_eq!(m.iter().fold(text, |acc, x| x.apply(acc)), expected)
    }
}
