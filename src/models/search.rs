use super::file_result::LineResult;
use super::{FileResult, Line, MatchResult, SearchCondition, SearchConfig};
use crate::models::search_result::SearchResult;
use colored::Colorize;
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use itertools::Itertools;
use rayon::prelude::*;
use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct Search {
    path_list: Vec<String>,
    stdin: Option<String>,
}

impl Search {
    pub fn new(path_list: Vec<String>, stdin: Option<String>) -> Self {
        Self { path_list, stdin }
    }

    pub fn search(&self, config: SearchConfig, conditions: Vec<SearchCondition>) -> SearchResult {
        match self.stdin.clone() {
            Some(_) => self.search_stdin(config, conditions),
            None => self.search_files(config, conditions),
        }
    }

    fn search_files(&self, config: SearchConfig, conditions: Vec<SearchCondition>) -> SearchResult {
        if conditions.is_empty() {
            return SearchResult::new(vec![], vec![]);
        }

        let find = conditions.iter().find(|c| c.is_matcher());
        let line_filter = conditions.iter().find(|c| c.is_line_filter());

        SearchResult::new(
            self.path_list
                .iter()
                .flat_map(|path| {
                    self.entries(path, &config)
                        .par_iter()
                        .filter_map(|path| {
                            fs::read_to_string(path)
                                .ok()
                                .map(|content| {
                                    self.search_content(content, &config, find, line_filter)
                                })
                                .and_then(|lines| {
                                    if lines.is_empty() {
                                        None
                                    } else {
                                        Some(FileResult {
                                            file_path: path.to_string(),
                                            lines,
                                        })
                                    }
                                })
                        })
                        .collect::<Vec<FileResult>>()
                })
                .collect::<Vec<FileResult>>(),
            conditions,
        )
    }

    fn search_stdin(&self, config: SearchConfig, conditions: Vec<SearchCondition>) -> SearchResult {
        if conditions.is_empty() {
            return SearchResult::new(vec![], vec![]);
        }

        let find = conditions.iter().find(|c| c.is_matcher());
        let line_filter = conditions.iter().find(|c| c.is_line_filter());

        match self.stdin.clone() {
            Some(stdin) => SearchResult::new(
                vec![FileResult {
                    // TODO: fix me
                    file_path: "".to_string(),
                    lines: self.search_content(stdin, &config, find, line_filter),
                }],
                conditions,
            ),
            None => SearchResult::new(
                vec![FileResult {
                    // TODO: fix me
                    file_path: "".to_string(),
                    lines: vec![],
                }],
                conditions,
            ),
        }
    }

    fn search_content(
        &self,
        content: String,
        config: &SearchConfig,
        find: Option<&SearchCondition>,
        line_filter: Option<&SearchCondition>,
    ) -> Vec<LineResult> {
        let slice = content.lines().collect::<Vec<_>>();

        content
            .lines()
            .enumerate()
            .flat_map(|(index, line)| {
                if let Some(f) = &line_filter {
                    if let Some(line_filter) = f.line_filter() {
                        if !line_filter.filter(line) {
                            return vec![];
                        }
                    }
                }

                if let Some(f) = &find {
                    f.matcher().and_then(|f| {
                        let line = line.replace('\t', " ");
                        let matches = MatchResult::find(line.to_string(), f, 1);

                        if matches.is_empty() {
                            None
                        } else {
                            let before = if let Some(before) = config.before_context {
                                let start = if index < before { 0 } else { index - before };
                                slice[start..index]
                                    .iter()
                                    .enumerate()
                                    .map(|(i, it)| {
                                        LineResult::Line(Line::new(
                                            start + i,
                                            it.to_string(),
                                            vec![],
                                            false,
                                        ))
                                    })
                                    .collect_vec()
                            } else {
                                vec![]
                            };

                            let after = if let Some(after) = config.after_context {
                                let end = if index + after >= slice.len() {
                                    slice.len()
                                } else {
                                    index + after + 1
                                };
                                slice[index + 1..end]
                                    .iter()
                                    .enumerate()
                                    .map(|(i, it)| {
                                        LineResult::Line(Line::new(
                                            index + i,
                                            it.to_string(),
                                            vec![],
                                            false,
                                        ))
                                    })
                                    .collect_vec()
                            } else {
                                vec![]
                            };

                            Some(itertools::concat(vec![
                                before.clone(),
                                vec![LineResult::Line(Line::new(
                                    index + 1,
                                    line.to_string(),
                                    matches,
                                    false,
                                ))],
                                after.clone(),
                                if before.len() + after.len() > 1 {
                                    vec![LineResult::Separator]
                                } else {
                                    vec![]
                                },
                            ]))
                        }
                    })
                } else {
                    Some(vec![LineResult::Line(Line::new(
                        index + 1,
                        line.to_string(),
                        Vec::new(),
                        false,
                    ))])
                }
                .unwrap_or_default()
            })
            .collect::<Vec<_>>()
    }

    fn entries(&self, path: &str, config: &SearchConfig) -> Vec<String> {
        if Path::new(path).is_file() {
            vec![path.to_string()]
        } else {
            let mut walk_builder = WalkBuilder::new(path);
            let mut overrides_builder = OverrideBuilder::new(".");

            if let Some(exclude) = &config.exclude_path {
                overrides_builder
                    .add(format!("!{}", exclude).as_str())
                    .unwrap();
                walk_builder.overrides(overrides_builder.build().unwrap());
            }

            walk_builder
                .git_ignore(!config.no_git_ignore)
                .git_exclude(!config.no_git_exclude)
                .hidden(config.hidden)
                .max_depth(config.max_depth)
                .build()
                .filter_map(|entry| {
                    entry
                        .map_err(|err| eprintln!("{}", err.to_string().bold().red()))
                        .ok()
                })
                .map(|entry| entry.path().to_str().unwrap().to_string())
                .collect_vec()
        }
    }
}
