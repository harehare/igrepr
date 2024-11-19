use super::{MatchColors, Theme};
use crate::models::file_result::LineResult;
use crate::models::{self, MatchResult, SearchResultConfig};
use crate::ui;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use nom::ToUsize;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
    Frame,
};
use std::sync::{mpsc, Arc};

#[derive(Clone)]
pub struct SearchResult {
    rows: Vec<Row>,
    state: ListState,
    config: SearchResultConfig,
    tx: mpsc::Sender<ui::Event>,
}

#[derive(Clone)]
enum Row {
    File(models::FileResult),
    Line(models::FileResult, models::Line),
    Separator,
}

impl SearchResult {
    pub fn new(
        files: &[models::FileResult],
        config: SearchResultConfig,
        tx: mpsc::Sender<ui::Event>,
    ) -> Self {
        let rows = files
            .iter()
            .flat_map(|file| {
                let rows = if config.vimgrep {
                    file.lines
                        .iter()
                        .flat_map(|line| {
                            if let LineResult::Line(line) = line {
                                if line.is_filtered() {
                                    Vec::new()
                                } else {
                                    line.matches()
                                        .iter()
                                        .map(|m| {
                                            Row::Line(
                                                file.clone(),
                                                models::Line::new(
                                                    line.line_no,
                                                    line.text.clone(),
                                                    vec![m.clone()],
                                                    line.is_filtered(),
                                                ),
                                            )
                                        })
                                        .collect::<Vec<_>>()
                                }
                            } else {
                                vec![Row::Separator]
                            }
                        })
                        .collect::<Vec<_>>()
                } else {
                    file.lines
                        .iter()
                        .filter_map(|line| {
                            if let LineResult::Line(line) = line {
                                if line.is_filtered() {
                                    None
                                } else {
                                    Some(Row::Line(file.clone(), line.clone()))
                                }
                            } else {
                                Some(Row::Separator)
                            }
                        })
                        .collect::<Vec<_>>()
                };

                if rows.is_empty() {
                    Vec::new()
                } else {
                    itertools::concat(vec![vec![Row::File(file.clone())], rows])
                }
            })
            .collect();
        let mut state = ListState::default();
        state.select(if files.is_empty() { None } else { Some(1) });

        Self {
            rows,
            state,
            config,
            tx,
        }
    }

    fn next(&mut self) {
        if self.rows.is_empty() {
            return;
        }

        let next_index = self
            .state
            .selected()
            .map(|i| {
                if i == self.rows.len() - 1 && !self.rows.is_empty() {
                    1
                } else {
                    match self.rows[i + 1] {
                        Row::File(_) => i + 2,
                        Row::Line(_, _) => i + 1,
                        Row::Separator => i + 2,
                    }
                }
            })
            .unwrap_or(0);

        self.state.select(Some(next_index));

        let (file, line) = self.selected().unwrap_or_else(|| {
            self.state.select(Some(next_index + 1));
            self.selected().unwrap()
        });

        self.tx
            .clone()
            .send(ui::Event::ChangeResultLine(file, line))
            .ok();
    }

    fn next_file(&mut self) {
        if self.rows.is_empty() {
            return;
        }

        let next_index = self
            .state
            .selected()
            .map(|i| {
                if i == self.rows.len() - 1 && !self.rows.is_empty() {
                    1
                } else {
                    match self.rows[i + 1] {
                        Row::File(_) => i + 2,
                        _ => self.rows[i + 1..]
                            .iter()
                            .position(|r| matches!(r, Row::File(_)))
                            .map(|p| i + p + 2)
                            .unwrap_or_else(|| self.rows.len() - 1),
                    }
                }
            })
            .unwrap_or(0);

        self.state.select(Some(next_index));

        let (file, line) = self.selected().unwrap();
        self.tx
            .clone()
            .send(ui::Event::ChangeResultLine(file, line))
            .ok();
    }

    fn previous_file(&mut self) {
        if self.rows.is_empty() {
            return;
        }

        let prev_index = self
            .state
            .selected()
            .and_then(|i| {
                let v = match self.rows[i.checked_sub(1).unwrap_or_default()] {
                    Row::File(_) => i.checked_sub(2).unwrap_or_default(),
                    _ => i.checked_sub(1).unwrap_or_default(),
                };

                self.rows[..v]
                    .iter()
                    .rev()
                    .position(|r| matches!(r, Row::File(_)))
                    .map(|p| (v - p))
            })
            .unwrap_or(self.rows.len() - 1);

        self.state.select(Some(prev_index));

        let (file, line) = self.selected().unwrap();
        self.tx
            .clone()
            .send(ui::Event::ChangeResultLine(file, line))
            .ok();
    }

    fn previous(&mut self) {
        if self.rows.is_empty() {
            return;
        }

        let prev_index = self
            .state
            .selected()
            .and_then(|i| match self.rows[i.checked_sub(1).unwrap_or_default()] {
                Row::File(_) => i.checked_sub(2),
                Row::Line(_, _) => i.checked_sub(1),
                Row::Separator => i.checked_sub(2),
            })
            .unwrap_or(self.rows.len() - 1);

        self.state.select(Some(prev_index));

        let (file, line) = self.selected().unwrap_or_else(|| {
            self.state.select(Some(prev_index - 1));
            self.selected().unwrap()
        });
        self.tx
            .clone()
            .send(ui::Event::ChangeResultLine(file, line))
            .ok();
    }

    pub fn selected(&mut self) -> Option<(models::FileResult, models::Line)> {
        if self.rows.is_empty() {
            None
        } else {
            self.state.selected().and_then(|i| match self.rows.get(i) {
                Some(Row::Line(f, l)) => Some((f.clone(), l.clone())),
                _ => None,
            })
        }
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect, theme: Arc<dyn Theme>) {
        let list_items: Vec<ListItem> = self
            .rows
            .iter()
            .filter_map(|row| match row {
                Row::File(file) if !self.config.no_file_name && !self.config.vimgrep => {
                    Some(ListItem::new(Line::from(vec![
                        Span::styled(file.file_path.clone(), theme.file_style()),
                        Span::styled(
                            format!(" [{}]", file.lines.len()),
                            theme.count_style().add_modifier(Modifier::BOLD),
                        ),
                    ])))
                }
                Row::File(_) => None,
                Row::Line(f, line) if self.config.vimgrep => {
                    let line_no = vec![
                        Span::styled(f.file_path.clone(), theme.file_style()),
                        Span::styled(
                            format!(
                                ":{}:{}:",
                                line.line_no + 1,
                                line.matches()
                                    .first()
                                    .map(|v| v.column())
                                    .unwrap_or_default()
                            ),
                            theme.line_no_style(),
                        ),
                    ];

                    Some(ListItem::new(Line::from(itertools::concat(vec![
                        line_no,
                        line.tokens()
                            .into_iter()
                            .flat_map(|(token, m)| match m {
                                Some(MatchResult::Found(_, _, index)) => {
                                    vec![Span::styled(
                                        token,
                                        Style::default()
                                            .fg(MatchColors::get_color(index.to_usize()))
                                            .add_modifier(Modifier::BOLD),
                                    )]
                                }
                                Some(MatchResult::Transformed(text, _, index)) => {
                                    vec![
                                        Span::styled(token, theme.disabled_style()),
                                        Span::styled(
                                            text.clone(),
                                            Style::default()
                                                .fg(MatchColors::get_color(index.to_usize()))
                                                .add_modifier(Modifier::BOLD),
                                        ),
                                    ]
                                }

                                _ => vec![Span::raw(token)],
                            })
                            .collect::<Vec<_>>(),
                    ]))))
                }
                Row::Line(_, line) => {
                    let line_no = if self.config.no_line_no {
                        Vec::new()
                    } else {
                        vec![Span::styled(
                            format!(" {}: ", line.line_no + 1),
                            theme.line_no_style(),
                        )]
                    };

                    Some(ListItem::new(Line::from(itertools::concat(vec![
                        line_no,
                        line.tokens()
                            .into_iter()
                            .flat_map(|(token, m)| match m {
                                Some(MatchResult::Found(_, _, index)) => {
                                    vec![Span::styled(
                                        token,
                                        Style::default()
                                            .fg(MatchColors::get_color(index.to_usize()))
                                            .add_modifier(Modifier::BOLD),
                                    )]
                                }
                                Some(MatchResult::Transformed(text, _, index)) => {
                                    vec![
                                        Span::styled(token, theme.disabled_style()),
                                        Span::styled(
                                            text,
                                            Style::default()
                                                .fg(MatchColors::get_color(index.to_usize()))
                                                .add_modifier(Modifier::BOLD),
                                        ),
                                    ]
                                }
                                _ => vec![Span::raw(token)],
                            })
                            .collect::<Vec<_>>(),
                    ]))))
                }
                Row::Separator => Some(ListItem::new(Line::from(vec![Span::styled(
                    self.config.context_separator.clone(),
                    theme.foreground_style(),
                )]))),
            })
            .collect::<Vec<_>>();

        if list_items.is_empty() {
            f.render_widget(
                Paragraph::new("Not Found")
                    .style(theme.foreground_style())
                    .block(Block::default().padding(Padding::top(2)))
                    .centered(),
                area,
            );
        } else {
            let list = List::new(list_items)
                .highlight_style(theme.highlight_style())
                .block(
                    Block::bordered()
                        .title("Search Result")
                        .title_style(theme.foreground_style()),
                );
            f.render_stateful_widget(list, area, &mut self.state);
        }
    }

    pub fn handle_event(&mut self, e: &Event) {
        if let Event::Key(key) = e {
            match key {
                KeyEvent {
                    code: KeyCode::Up, ..
                } => self.previous(),
                KeyEvent {
                    code: KeyCode::Down,
                    ..
                } => self.next(),
                KeyEvent {
                    code: KeyCode::PageUp,
                    ..
                } => self.previous_file(),
                KeyEvent {
                    code: KeyCode::PageDown,
                    ..
                } => self.next_file(),
                KeyEvent {
                    code: KeyCode::Char('r'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    self.state
                        .selected()
                        .and_then(|i| self.rows.get(i))
                        .map(|c| match c {
                            Row::Line(f, line) => self
                                .tx
                                .send(ui::Event::ReplaceSelectLine(f.clone(), line.clone()))
                                .ok(),
                            _ => None,
                        });
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    self.state
                        .selected()
                        .and_then(|i| self.rows.get(i))
                        .map(|c| match c {
                            Row::Line(f, line) => self
                                .tx
                                .send(ui::Event::SelectResultLine(f.clone(), line.clone()))
                                .ok(),
                            _ => None,
                        });
                }
                _ => (),
            }
        }
    }
}
