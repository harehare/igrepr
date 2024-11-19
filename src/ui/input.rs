use super::Theme;
use crate::{models, ui};
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    widgets::{Block, Padding, Paragraph},
    Frame,
};
use std::sync::{mpsc, Arc};
use tui_input::backend::crossterm::EventHandler;

#[derive(Clone, Debug)]
pub enum InputState {
    Input(Result<(), String>),
    Deleteable,
    Entered(models::SearchCondition),
}

#[derive(Clone, Debug)]
pub struct Input {
    input: tui_input::Input,
    input_state: InputState,
    tx: mpsc::Sender<ui::Event>,
}

impl Input {
    pub fn new(tx: mpsc::Sender<ui::Event>) -> Self {
        Self {
            input: tui_input::Input::default(),
            input_state: InputState::Input(Ok(())),
            tx,
        }
    }

    pub fn move_cursor(&self, index: usize) -> Self {
        Self {
            input: self.input.clone().with_cursor(index),
            ..self.clone()
        }
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect, theme: Arc<dyn Theme>) {
        match &self.input_state {
            InputState::Entered(_) => (),
            InputState::Deleteable => (),
            InputState::Input(Ok(_)) => {
                let input = Paragraph::new(self.input.value())
                    .block(Block::default().padding(Padding::new(1, 0, 1, 1)))
                    .style(theme.foreground_style());
                f.render_widget(input, area);
                f.set_cursor_position((
                    area.x + (self.input.visual_cursor()) as u16 + 1,
                    area.y + 1,
                ))
            }
            InputState::Input(Err(s)) => {
                self.tx
                    .clone()
                    .send(ui::Event::ShowMessage(Some(ui::Status::new(Some(
                        ui::Message::Error(s.to_owned()),
                    )))))
                    .ok();
                let input = Paragraph::new(self.input.value())
                    .block(Block::default().padding(Padding::new(0, 0, 1, 1)))
                    .style(theme.foreground_style());
                f.render_widget(input, area);
                f.set_cursor_position((
                    area.x + (self.input.visual_cursor()) as u16 + 1,
                    area.y + 1,
                ))
            }
        }
    }

    pub fn handle_event(&mut self, e: &Event) -> bool {
        let processed = if let Event::Key(key) = e {
            match key {
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => match &self.input_state {
                    InputState::Input(Ok(_)) if !self.input.value().is_empty() => {
                        match self.input.value().parse() {
                            Ok(c) => {
                                self.input_state = InputState::Entered(c);
                            }
                            Err(e) => {
                                self.input_state = InputState::Input(Err(e.to_string()));
                            }
                        };

                        true
                    }
                    InputState::Input(Err(e)) => {
                        self.input_state = InputState::Input(Err(e.to_string()));
                        true
                    }
                    _ => false,
                },
                KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                } if self.input.value().is_empty() => {
                    self.input_state = InputState::Deleteable;
                    true
                }

                _ => false,
            }
        } else {
            false
        };

        if !processed {
            self.input.handle_event(e);
        }

        processed
    }

    pub fn entered(condition: models::SearchCondition, tx: mpsc::Sender<ui::Event>) -> Self {
        Input {
            input: tui_input::Input::default().with_value(condition.value().unwrap_or_default()),
            input_state: InputState::Entered(condition),
            tx,
        }
    }

    pub fn entered_string(&self) -> Option<String> {
        match &self.input_state {
            InputState::Entered(s) => Some(s.to_string()),
            _ => None,
        }
    }

    pub fn entered_condition(&self) -> Option<models::SearchCondition> {
        match &self.input_state {
            InputState::Entered(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn input(&mut self) -> &Self {
        match &self.input_state {
            InputState::Entered(s) => {
                self.input = self.input.clone().with_value(s.to_string());
                self.input_state = InputState::Input(Ok(()));
            }
            InputState::Deleteable => self.input_state = InputState::Input(Ok(())),
            InputState::Input(_) => (),
        }

        self
    }

    pub fn is_entered(&self) -> bool {
        matches!(self.input_state, InputState::Entered(_))
    }

    pub fn is_deletable(&self) -> bool {
        matches!(self.input_state, InputState::Deleteable)
    }

    pub fn has_transform(&self) -> bool {
        match &self.input_state {
            InputState::Entered(c) => c.is_transform(),
            _ => false,
        }
    }

    pub fn set_condition(&mut self, condition: models::SearchCondition) {
        let value = self.input.value().to_string();
        self.input = self.input.clone().with_value(
            condition
                .with_value(self.input.value().to_string())
                .unwrap_or(models::SearchCondition::Exact(value))
                .to_string(),
        )
    }

    pub fn value(&self) -> &str {
        self.input.value()
    }
}
