use super::{Icon, Input, MatchColors, Theme};
use crate::{models::SearchCondition, ui};
use crossterm::event::{Event, KeyCode, KeyEvent};
use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};
use std::sync::{mpsc, Arc};

#[derive(Clone, Debug)]
pub struct InputList {
    current_input: Input,
    tx: mpsc::Sender<ui::Event>,
    pub entered_list: Vec<Input>,
}

impl InputList {
    pub fn new(entered_list: Vec<Input>, tx: mpsc::Sender<ui::Event>) -> Self {
        InputList {
            current_input: Input::new(tx.clone()),
            entered_list,
            tx,
        }
    }

    pub fn set_current_condition(&mut self, condition: SearchCondition) {
        if !&condition.has_args() {
            self.current_input = Input::entered(condition, self.tx.clone());
            self.update_entered_list();
        } else {
            self.current_input.set_condition(condition.clone());
            self.current_input = self
                .current_input
                .move_cursor(self.current_input.value().len() - 1);
        }
    }

    pub fn input_value(&self) -> &str {
        self.current_input.value()
    }

    pub fn has_transform(&self) -> bool {
        self.entered_list.iter().any(|i| i.has_transform())
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect, theme: Arc<dyn Theme>, icon: Arc<dyn Icon>) {
        let line = Line::from(itertools::concat(vec![self
            .entered_list
            .iter()
            .enumerate()
            .flat_map(|(index, i)| {
                i.entered_condition()
                    .map(|c| {
                        vec![
                            Span::styled(
                                ui::SearchCondition::new(c.clone(), icon.clone()).to_string(),
                                Style::default()
                                    .fg(MatchColors::get_color(index + 1))
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Span::from("|"),
                        ]
                    })
                    .unwrap_or_default()
            })
            .collect_vec()]));

        let entered_len = self.entered_list.iter().fold(0, |acc, x| {
            acc + x
                .entered_condition()
                .map(|c| {
                    ui::SearchCondition::new(c.clone(), icon.clone())
                        .to_string()
                        .len()
                })
                .unwrap_or_default()
        });

        let [icon_rect, entered_rect, current_input_rect] = Layout::horizontal([
            Constraint::Length(2),
            Constraint::Length(
                ((entered_len)
                    .checked_sub(self.entered_list.len().checked_sub(1).unwrap_or_default())
                    .unwrap_or_default()) as u16,
            ),
            Constraint::Percentage(100),
        ])
        .areas(area);

        f.render_widget(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style()),
            area,
        );
        f.render_widget(
            Paragraph::new(Span::styled(icon.search(), theme.foreground_style()))
                .block(Block::default().padding(Padding::new(1, 0, 1, 1))),
            icon_rect,
        );
        f.render_widget(
            Paragraph::new(line)
                .block(Block::default().padding(Padding::new(1, 0, 1, 1)))
                .style(theme.border_style()),
            entered_rect,
        );
        self.current_input.draw(f, current_input_rect, theme);
    }

    pub fn handle_event(&mut self, e: &Event) -> bool {
        let processed = self.current_input.handle_event(e);

        if let Event::Key(key) = e {
            match key {
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => self.update_entered_list() || processed,
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } if self.current_input.is_deletable() => {
                    self.current_input = self
                        .entered_list
                        .pop()
                        .map(|mut f| f.input().clone())
                        .unwrap_or_else(|| self.current_input.input().clone());
                    self.tx
                        .clone()
                        .send(ui::Event::DeleteSearchCondition(self.entered_list.len()))
                        .ok()
                        .map(|_| true)
                        .unwrap_or(true)
                }
                _ => processed,
            }
        } else {
            processed
        }
    }

    fn update_entered_list(&mut self) -> bool {
        if self.current_input.is_entered() {
            let i = self.current_input.clone();
            let tx = self.tx.clone();

            self.entered_list.push(self.current_input.clone());
            self.current_input = Input::new(self.tx.clone());

            if self.entered_list.len() == 1 {
                i.entered_condition()
                    .map(|c| tx.send(ui::Event::StartFileSearch(c)).ok());
            } else {
                i.entered_condition().map(|c| {
                    tx.send(ui::Event::StartResultSearch(c, self.entered_list.len()))
                        .ok()
                });
            }

            true
        } else {
            false
        }
    }
}
