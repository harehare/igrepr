use crate::{models::SearchCondition, ui};
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, ListState, Padding, Paragraph},
    Frame,
};
use std::sync::{mpsc, Arc};
use strum::IntoEnumIterator;
use tui_input::{backend::crossterm::EventHandler, Input};

use super::{Icon, Theme};

#[derive(Clone, Debug)]
pub struct SelectCondition {
    conditions: Vec<SearchCondition>,
    filtered_conditions: Vec<SearchCondition>,
    input: Input,
    state: ListState,
    tx: mpsc::Sender<ui::Event>,
}

impl SelectCondition {
    pub fn new(value: &str, tx: mpsc::Sender<ui::Event>) -> Self {
        let mut state = ListState::default();

        state.select(Some(0));
        SelectCondition {
            conditions: SearchCondition::iter()
                .filter(|c| !matches!(c, SearchCondition::Exact(_)))
                .map(|c| c.with_value(value.to_string()).unwrap_or(c))
                .collect(),
            filtered_conditions: Vec::new(),
            input: Input::default(),
            state,
            tx,
        }
    }

    pub fn draw(
        &mut self,
        f: &mut Frame,
        area: Rect,
        theme: Arc<dyn Theme>,
        first_search: bool,
        icon: Arc<dyn Icon>,
    ) {
        let [input_area, list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(area.height - 3)])
                .areas(area);

        let block = Block::bordered()
            .title("Select search conditions")
            .style(theme.popup_style());

        let input = Paragraph::new(self.input.value())
            .block(Block::bordered().border_style(theme.border_style()))
            .style(theme.foreround_style());

        self.filtered_conditions = self
            .conditions
            .iter()
            .filter_map(|condition| {
                if first_search && (condition.is_filter() || condition.is_transform()) {
                    None
                } else if (condition.to_string().is_empty()
                    && "exact()".contains(self.input.value()))
                    || self.input.value().is_empty()
                    || condition.to_string().contains(self.input.value())
                {
                    Some(condition.clone())
                } else {
                    None
                }
            })
            .collect();

        f.render_widget(Clear, area);
        f.render_widget(input, input_area);
        f.render_widget(block, area);

        if self.filtered_conditions.is_empty() {
            f.render_widget(
                Paragraph::new("Not found")
                    .block(Block::default().padding(Padding::new(1, 1, 0, 0)))
                    .style(theme.foreround_style()),
                list_area,
            );
        } else {
            let list_items = self
                .filtered_conditions
                .iter()
                .map(|condition| {
                    let s = if condition.to_string().is_empty() {
                        format!("{} exact()", icon.search())
                    } else {
                        ui::SearchCondition::new(condition.clone(), icon.clone()).to_string()
                    };

                    let style = if condition.is_matcher() {
                        theme.match_style()
                    } else if condition.is_filter() || condition.is_line_filter() {
                        theme.filter_style()
                    } else if condition.is_transform() {
                        theme.transform_style()
                    } else {
                        theme.foreround_style()
                    };

                    ListItem::new(Line::from(Span::styled(s, style)))
                })
                .collect::<Vec<_>>();
            let list = List::new(list_items)
                .highlight_style(theme.highlight_style())
                .block(Block::default().padding(Padding::new(1, 1, 0, 0)));
            f.render_stateful_widget(list, list_area, &mut self.state);
        }

        f.set_cursor_position((area.x + (self.input.visual_cursor()) as u16 + 1, area.y + 1))
    }

    pub fn handle_event(&mut self, e: &Event) {
        if let Event::Key(key) = e {
            match key {
                KeyEvent {
                    code: KeyCode::Down,
                    ..
                } => {
                    let next_index = self
                        .state
                        .selected()
                        .map(|i| {
                            if i == self.filtered_conditions.len() - 1
                                && !self.conditions.is_empty()
                            {
                                1
                            } else {
                                i + 1
                            }
                        })
                        .unwrap_or(0);

                    self.state.select(Some(next_index));
                }
                KeyEvent {
                    code: KeyCode::Up, ..
                } => {
                    let prev_index = self
                        .state
                        .selected()
                        .and_then(|i| i.checked_sub(1))
                        .unwrap_or(self.filtered_conditions.len() - 1);

                    self.state.select(Some(prev_index));
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => {
                    self.state
                        .selected()
                        .and_then(|i| self.filtered_conditions.get(i))
                        .and_then(|c| self.tx.send(ui::Event::SelectCondition(c.clone())).ok());
                }
                _ => {
                    self.state.select(Some(0));
                    self.input.handle_event(e);
                }
            }
        }
    }
}
