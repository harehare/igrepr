use super::MatchResult;
use crate::{filter::Filter, ui::MatchColors};
use colored::Colorize;
use colored::{self, ColoredString};
use itertools::Itertools;
use std::{
    fmt::{self, Display, Formatter},
    sync::Arc,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub line_no: usize,
    pub text: String,
    matches: Vec<MatchResult>,
    filtered: bool,
}

impl Line {
    pub fn new(
        line_no: usize,
        text: String,
        mut matches: Vec<MatchResult>,
        filtered: bool,
    ) -> Self {
        matches.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Line {
            line_no,
            text,
            matches,
            filtered,
        }
    }

    pub fn is_filtered(&self) -> bool {
        self.filtered
    }

    pub fn matches(&self) -> &Vec<MatchResult> {
        &self.matches
    }

    pub fn transforms(&self) -> Vec<MatchResult> {
        self.matches
            .clone()
            .into_iter()
            .filter(|m| m.is_transformed())
            .collect_vec()
    }

    pub fn contains_transformed(&self) -> bool {
        self.matches.iter().any(|m| m.is_transformed())
    }

    pub fn filter(&self, filter: Arc<dyn Filter>) -> Line {
        Line {
            line_no: self.line_no,
            text: self.text.clone(),
            matches: self.matches.clone(),
            filtered: !filter.filter(&self.text),
        }
    }

    pub fn filtered(&self, filtered: bool) -> Line {
        Line {
            line_no: self.line_no,
            text: self.text.clone(),
            matches: self.matches.clone(),
            filtered,
        }
    }

    pub fn count_matches(&self) -> usize {
        self.matches.iter().filter(|m| m.is_found()).count()
    }

    pub fn tokens(&self) -> Vec<(String, Option<MatchResult>)> {
        let mut current_index = 0;
        let mut tokens: Vec<(String, Option<MatchResult>)> = vec![];
        let matches = self.matches.clone();

        for m in matches {
            match m.clone() {
                MatchResult::Found(_, range, _) => {
                    if current_index < range.start {
                        tokens.push((self.text[current_index..range.start].to_string(), None));
                    }

                    if current_index > range.start && current_index <= range.end {
                        tokens.push((self.text[current_index..range.end].to_string(), Some(m)));
                    } else {
                        tokens.push((self.text[range.start..range.end].to_string(), Some(m)));
                    }

                    current_index = range.end;
                }
                MatchResult::Filtered(_, range, _) => {
                    if current_index < range.start {
                        tokens.push((self.text[current_index..range.start].to_string(), None));
                    }
                    tokens.push((self.text[range.start..range.end].to_string(), Some(m)));
                    current_index = range.end;
                }
                MatchResult::Transformed(_, range, _) => {
                    if current_index < range.start {
                        tokens.push((self.text[current_index..range.start].to_string(), None));
                    }
                    tokens.push((self.text[range.start..range.end].to_string(), Some(m)));
                    current_index = range.end;
                }
            }
        }

        if self.matches.is_empty() {
            tokens.push((self.text.clone(), None));
        } else if !self.text.is_empty() && current_index < self.text.len() {
            tokens.push((self.text[current_index..self.text.len()].to_string(), None));
        }
        tokens
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:", self.line_no)?;
        for c in self.tokens().into_iter().flat_map(|(token, m)| match m {
            Some(MatchResult::Found(_, _, index)) => {
                vec![token
                    .to_string()
                    .color(MatchColors::get_cli_color(index))
                    .bold()]
            }
            Some(MatchResult::Transformed(text, _, index)) => {
                vec![
                    token.to_string().strikethrough(),
                    text.to_string()
                        .color(MatchColors::get_cli_color(index))
                        .bold(),
                ]
            }
            _ => vec![ColoredString::from(token.to_string())],
        }) {
            write!(f, "{}", c)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{filter::InvertMatch, models::MatchResult};
    use rstest::rstest;
    use std::{ops::Range, vec};

    #[rstest]
    #[case(vec![MatchResult::Found("string".to_string(), Range{start: 5, end: 11}, 1),
                MatchResult::Transformed("test".to_string(), Range{start: 0, end: 4}, 2),
                MatchResult::Filtered("test".to_string(), Range{start: 0, end: 4}, 3),
                MatchResult::Found("test".to_string(), Range{start: 0, end: 4}, 1)],
           vec![MatchResult::Found("test".to_string(), Range{start: 0, end: 4}, 1),
                MatchResult::Transformed("test".to_string(), Range{start: 0, end: 4}, 2),
                MatchResult::Filtered("test".to_string(), Range{start: 0, end: 4}, 3),
                MatchResult::Found("string".to_string(), Range{start: 5, end: 11}, 1),
           ])]
    fn test_new(#[case] matches: Vec<MatchResult>, #[case] expected: Vec<MatchResult>) {
        assert_eq!(
            Line::new(0, "".to_string(), matches, false)
                .matches()
                .clone(),
            expected
        )
    }

    #[rstest]
    #[case(Line {
            line_no: 1,
            text: "test string end😄".to_string(),
            matches: vec![MatchResult::Found("test".to_string(), Range{start: 0, end: 4}, 1),
                          MatchResult::Found("string".to_string(), Range{start: 5, end: 11}, 1)],
            filtered: false},
            vec![("test".to_string(),
                  Some(MatchResult::Found("test".to_string(), Range{start: 0, end: 4}, 1))),
                  (" ".to_string(), None),
                  ("string".to_string(), Some(MatchResult::Found("string".to_string(), Range{start: 5, end: 11}, 1))),
                  (" end😄".to_string(), None),
                ])]
    fn test_tokens(#[case] line: Line, #[case] expected: Vec<(String, Option<MatchResult>)>) {
        assert_eq!(line.tokens(), expected)
    }

    #[rstest]
    #[case(Line {
            line_no: 1,
            text: "test string".to_string(),
            matches: vec![MatchResult::Found("test".to_string(), Range{start: 0, end: 4}, 1),
                          MatchResult::Found("string".to_string(), Range{start: 5, end: 11}, 1)],
            filtered: false},
            InvertMatch::new("string".to_string()),
            true)]
    fn test_filter(#[case] line: Line, #[case] filter: Arc<dyn Filter>, #[case] expected: bool) {
        assert_eq!(line.filter(filter).filtered, expected)
    }
}
