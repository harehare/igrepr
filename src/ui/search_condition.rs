use super::{Icon, MatchColors};
use crate::models;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Padding, Paragraph},
    Frame,
};
use std::{fmt::Display, sync::Arc};

pub struct SearchCondition<'a> {
    condition: models::SearchCondition,
    icon: Arc<dyn Icon<'a>>,
}

impl<'a> SearchCondition<'a> {
    pub fn new(condition: models::SearchCondition, icon: Arc<dyn Icon<'a>>) -> Self {
        SearchCondition { condition, icon }
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect, index: usize) {
        let input = Paragraph::new(Span::styled(
            self.condition.to_string(),
            Style::default()
                .fg(MatchColors::get_color(index))
                .add_modifier(Modifier::BOLD),
        ))
        .block(Block::default().padding(Padding::new(1, 0, 1, 1)));
        f.render_widget(input, area);
    }
}

impl Display for SearchCondition<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match &self.condition {
            models::SearchCondition::Exact(s) => format!("{} {}", self.icon.search(), s),
            models::SearchCondition::IgnoreCase(_) => {
                format!("{}{}", self.icon.ignore_case(), &self.condition.to_string(),)
            }
            models::SearchCondition::Number(_) => {
                format!("{} {}", self.icon.number(), &self.condition.to_string(),)
            }
            models::SearchCondition::WholeWord(_) => {
                format!("{}{}", self.icon.whole_word(), &self.condition.to_string(),)
            }
            models::SearchCondition::Regex(_) => {
                format!("{}{}", self.icon.regex(), &self.condition.to_string(),)
            }
            models::SearchCondition::Replace(_, _) => {
                format!("{}{}", self.icon.replace(), &self.condition.to_string(),)
            }
            models::SearchCondition::Insert(_, _) => {
                format!("{} {}", self.icon.insert(), &self.condition.to_string(),)
            }
            models::SearchCondition::Delete(_, _) => {
                format!("{} {}", self.icon.delete(), &self.condition.to_string(),)
            }
            _ if self.condition.is_matcher() => {
                format!("{} {}", self.icon.search(), &self.condition.to_string())
            }
            _ if self.condition.is_line_filter() => {
                format!("{} {}", self.icon.line(), &self.condition.to_string())
            }
            _ if self.condition.is_filter() => {
                format!("{} {}", self.icon.filter(), &self.condition.to_string())
            }
            _ if self.condition.is_transform() => {
                format!("{}{}", self.icon.replace(), &self.condition.to_string())
            }
            _ => self.condition.to_string().to_string(),
        };
        write!(f, "{}", s)
    }
}
