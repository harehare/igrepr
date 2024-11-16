use super::Theme;
use crate::models::Stat;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};
use std::sync::Arc;

#[derive(Clone)]
pub enum Message {
    Info(String),
    Warn(String),
    Error(String),
    Stat(Stat),
}

#[derive(Clone, Default)]
pub struct Status {
    message: Option<Message>,
}

impl Status {
    pub fn new(message: Option<Message>) -> Self {
        Self { message }
    }

    pub fn draw(&self, f: &mut Frame, area: Rect, theme: Arc<dyn Theme>) {
        match &self.message {
            Some(Message::Info(i)) => {
                let line = Line::from(vec![
                    Span::styled(" INFO ", theme.info_style()),
                    Span::styled(format!(" {} ", i), theme.status_style()),
                ]);

                f.render_widget(
                    Paragraph::new(line).block(Block::default().style(theme.status_style())),
                    area,
                );
            }
            Some(Message::Warn(w)) => {
                let line = Line::from(vec![
                    Span::styled(" WARN ", theme.warn_style()),
                    Span::styled(format!(" {} ", w), theme.status_style()),
                ]);

                f.render_widget(
                    Paragraph::new(line).block(Block::default().style(theme.status_style())),
                    area,
                );
            }
            Some(Message::Error(e)) => {
                let line = Line::from(vec![
                    Span::styled(" ERROR ", theme.error_style()),
                    Span::styled(format!(" {} ", e), theme.status_style()),
                ]);

                f.render_widget(
                    Paragraph::new(line).block(Block::default().style(theme.status_style())),
                    area,
                );
            }
            Some(Message::Stat(s)) => {
                let line = Line::from(vec![
                    Span::styled(" INFO ", theme.info_style()),
                    Span::styled(
                        format!(" Found {} matches in {} files", s.match_count, s.file_count),
                        theme.status_style(),
                    ),
                ]);

                f.render_widget(
                    Paragraph::new(line).block(Block::default().style(theme.status_style())),
                    area,
                );
            }
            _ => (),
        }
    }
}
