use super::file_result::LineResult;
use super::{FileResult, Line, MatchResult, SearchCondition};
use crate::ui;
use anyhow::Result;
use itertools::{concat, Itertools};
use rayon::prelude::*;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc;
use std::{fmt, fs};

#[derive(Clone)]
pub struct SearchResult {
    pub files: Vec<FileResult>,
    conditions: Vec<SearchCondition>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stat {
    pub file_count: usize,
    pub match_count: usize,
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            &self.files.iter().map(|f| format!("{}", f)).join("\n")
        )
    }
}

impl SearchResult {
    pub fn new(files: Vec<FileResult>, conditions: Vec<SearchCondition>) -> Self {
        Self { files, conditions }
    }

    pub fn stat(&self) -> Stat {
        Stat {
            file_count: self.files.len(),
            match_count: self.files.iter().fold(0, |acc, f| {
                acc + f.lines.iter().fold(0, |acc, l| match l {
                    LineResult::Line(l) => acc + l.matches().iter().count(),
                    LineResult::Separator => acc,
                })
            }),
        }
    }

    pub fn reapply(&self) -> SearchResult {
        self.conditions
            .iter()
            .enumerate()
            .fold(self.clear(), |acc, (index, f)| {
                acc.apply(f.clone(), index + 1)
            })
    }

    pub fn delete_last_condition(&mut self) -> SearchResult {
        let _ = self.conditions.pop();
        self.reapply()
    }

    fn clear(&self) -> SearchResult {
        SearchResult {
            files: self
                .files
                .iter()
                .map(|file| FileResult {
                    file_path: file.file_path.clone(),
                    lines: file
                        .lines
                        .iter()
                        .map(|line| match line {
                            LineResult::Line(line) => LineResult::Line(Line::new(
                                line.line_no,
                                line.text.clone(),
                                Vec::new(),
                                false,
                            )),
                            LineResult::Separator => LineResult::Separator,
                        })
                        .collect(),
                })
                .collect(),
            conditions: self.conditions.clone(),
        }
    }

    pub fn reflect(&self, tx: mpsc::Sender<ui::Event>) -> Result<()> {
        self.files
            .par_iter()
            .map(|file| self.reflect_file(file, tx.clone()))
            .collect::<Result<Vec<_>>>()?;

        Ok(())
    }

    pub fn apply(&self, c: SearchCondition, index: usize) -> SearchResult {
        SearchResult {
            files: self
                .files
                .par_iter()
                .map(|file| FileResult {
                    file_path: file.file_path.clone(),
                    lines: file
                        .lines
                        .iter()
                        .map(|line| {
                            if let LineResult::Line(line) = line {
                                let matches = concat(vec![
                                    line.matches().clone(),
                                    c.matcher()
                                        .map(|f| {
                                            MatchResult::find(
                                                line.text.to_string(),
                                                f.clone(),
                                                index,
                                            )
                                            .into_iter()
                                            .filter_map(|m| {
                                                if line.matches().contains(&m) {
                                                    None
                                                } else {
                                                    Some(m)
                                                }
                                            })
                                            .collect_vec()
                                        })
                                        .unwrap_or_default(),
                                ]);

                                let line = if !line.is_filtered() {
                                    line.filtered(c.is_matcher() && matches.is_empty())
                                } else {
                                    line.filtered(true)
                                };

                                let matches = c
                                    .match_filter()
                                    .map(|f| {
                                        matches
                                            .iter()
                                            .filter_map(|m| m.filter(f.clone(), index))
                                            .collect()
                                    })
                                    .unwrap_or(matches);
                                let matches = c
                                    .transform()
                                    .map(|f| {
                                        matches
                                            .iter()
                                            .flat_map(|m| m.transform(f.clone(), index))
                                            .collect()
                                    })
                                    .unwrap_or(matches);
                                let line_filter = c.line_filter();
                                let line = Line::new(
                                    line.line_no,
                                    line.text.clone(),
                                    matches,
                                    line.is_filtered(),
                                );

                                LineResult::Line(
                                    line_filter.map(|f| line.filter(f)).unwrap_or(line),
                                )
                            } else {
                                LineResult::Separator
                            }
                        })
                        .collect(),
                })
                .collect(),
            conditions: concat(vec![self.conditions.clone(), vec![c]]),
        }
    }

    pub fn to_conditions_string(&self) -> String {
        self.conditions.iter().map(|s| s.to_string()).join(" | ")
    }

    pub fn reflect_on_selected_row(&mut self, file_result: &FileResult, line: &Line) -> Result<()> {
        if file_result.contains_transformed() {
            let text = fs::read_to_string(&file_result.file_path)?;
            let mut lines = text.lines().map(|s| s.to_string()).collect_vec();

            for m in line.matches() {
                lines[line.line_no - 1] = m.apply(lines[line.line_no - 1].to_string());
            }

            let mut file = File::create(&file_result.file_path)?;
            file.write_all(lines.join("\n").as_bytes())?;

            self.files = self
                .files
                .clone()
                .into_iter()
                .map(|f| {
                    if f.file_path == file_result.file_path {
                        FileResult {
                            file_path: f.file_path.clone(),
                            lines: f
                                .lines
                                .into_iter()
                                .filter(|l| {
                                    if let LineResult::Line(l) = l {
                                        l.line_no != line.line_no
                                    } else {
                                        true
                                    }
                                })
                                .collect_vec(),
                        }
                    } else {
                        f
                    }
                })
                .collect_vec();
        }

        Ok(())
    }

    fn reflect_file(&self, file: &FileResult, tx: mpsc::Sender<ui::Event>) -> Result<()> {
        let text = fs::read_to_string(&file.file_path)?;
        let mut lines = text.lines().map(|s| s.to_string()).collect_vec();

        for line in &file.lines {
            if let LineResult::Line(line) = line {
                for m in line.transforms() {
                    lines[line.line_no - 1] = m.apply(lines[line.line_no - 1].to_string());
                }
            }
        }

        let mut file = File::create(&file.file_path)?;
        file.write_all(lines.join("\n").as_bytes())?;
        tx.send(ui::Event::Progress(1)).map_err(anyhow::Error::from)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::MatchResult;

    use super::*;
    use rstest::rstest;
    use std::{ops::Range, vec};

    #[rstest]
    #[case(SearchResult {files: Vec::new(), conditions: Vec::new()}, SearchCondition::Exact("test".to_string()), Vec::new())]
    #[case(SearchResult {
             files: vec![FileResult {file_path: "test".to_string(),
             lines: vec![LineResult::Line(Line::new(1, "test string".to_string(), Vec::new(), false))]}],
             conditions: Vec::new()},
             SearchCondition::Exact("test".to_string()),
             vec![MatchResult::Found("test".to_string(), Range{start: 0, end: 4}, 1)])]
    fn apply_test1(
        #[case] result: SearchResult,
        #[case] condition: SearchCondition,
        #[case] expected: Vec<MatchResult>,
    ) {
        assert_eq!(
            result
                .apply(condition, 1)
                .files
                .into_iter()
                .flat_map(|file| file
                    .lines
                    .into_iter()
                    .flat_map(|line| if let LineResult::Line(line) = line {
                        line.matches().clone()
                    } else {
                        Vec::new()
                    })
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            expected
        )
    }

    #[rstest]
    #[case(SearchResult {files: Vec::new(), conditions: Vec::new()}, SearchCondition::StartsWith("test".to_string()), None)]
    #[case(SearchResult {
             files: vec![FileResult {file_path: "test".to_string(),
             lines: vec![
                LineResult::Line(Line::new(1, "test string".to_string(), Vec::new(), false))
             ]}],
             conditions: Vec::new()},
             SearchCondition::LineStartsWith("te".to_string()), Some(Line::new(1, "test string".to_string(), Vec::new(), false)))]
    #[case(SearchResult {
             files: vec![FileResult {file_path: "test".to_string(),
             lines: vec![
                LineResult::Line(Line::new(1, "test string".to_string(), Vec::new(), false))
             ]}],
             conditions: Vec::new()},
             SearchCondition::LineEndsWith("st".to_string()), Some(Line::new(1, "test string".to_string(), Vec::new(), true)))]
    #[case(SearchResult {
             files: vec![FileResult {file_path: "test".to_string(),
             lines: vec![
                LineResult::Line(Line::new(1, "test string".to_string(), Vec::new(), false))
             ]}],
             conditions: Vec::new()},
             SearchCondition::LineInvertMatch("st".to_string()), Some(Line::new(1, "test string".to_string(), Vec::new(), true)))]
    fn apply_line_filter_test(
        #[case] result: SearchResult,
        #[case] condition: SearchCondition,
        #[case] expected: Option<Line>,
    ) {
        assert_eq!(
            result
                .apply(condition, 1)
                .files
                .into_iter()
                .flat_map(|file| file.lines)
                .collect::<Vec<_>>()
                .first()
                .and_then(|it| {
                    if let LineResult::Line(line) = it {
                        Some(line.clone())
                    } else {
                        None
                    }
                }),
            expected
        )
    }

    #[rstest]
    #[case(SearchResult {files: Vec::new(), conditions: Vec::new()}, SearchCondition::StartsWith("test".to_string()), Vec::new())]
    #[case(SearchResult {
             files: vec![FileResult {file_path: "test".to_string(),
             lines: vec![
                LineResult::Line(Line::new(1, "test string".to_string(), vec![MatchResult::Found("test".to_string(), Range{start: 0, end: 4}, 1)], false))
             ]}],
             conditions: vec![SearchCondition::Exact("test".to_string())]},
             SearchCondition::StartsWith("te".to_string()), vec![MatchResult::Filtered("test".to_string(), Range{start: 0, end: 4}, 1)])]
    #[case(SearchResult {
             files: vec![FileResult {file_path: "test".to_string(),
             lines: vec![
                LineResult::Line(Line::new(1, "test string".to_string(), vec![MatchResult::Found("test".to_string(), Range{start: 0, end: 4}, 1)], false))
             ]}],
             conditions: vec![SearchCondition::Exact("test".to_string())]},
             SearchCondition::EndsWith("st".to_string()), vec![MatchResult::Filtered("test".to_string(), Range{start: 0, end: 4}, 1)])]
    fn apply_match_filter_test(
        #[case] result: SearchResult,
        #[case] condition: SearchCondition,
        #[case] expected: Vec<MatchResult>,
    ) {
        assert_eq!(
            result
                .apply(condition, 1)
                .files
                .into_iter()
                .flat_map(|file| file
                    .lines
                    .into_iter()
                    .flat_map(|line| if let LineResult::Line(line) = line {
                        line.matches().clone()
                    } else {
                        Vec::new()
                    })
                    .collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            expected
        )
    }

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
