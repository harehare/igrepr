use super::Theme;
use crate::ui;
use anyhow::Result;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Clear, Padding, Paragraph},
    Frame,
};
use std::sync::{mpsc, Arc};

#[derive(Clone, Debug)]
pub enum Button {
    Yes,
    No,
}

#[derive(Clone, Debug)]
pub struct Confirm {
    id: String,
    title: String,
    text: String,
    yes_button: String,
    no_button: String,
    selected_button: Button,
    tx: mpsc::Sender<ui::Event>,
}

impl Confirm {
    pub fn new(
        id: String,
        title: String,
        text: String,
        yes_button: String,
        no_button: String,
        tx: mpsc::Sender<ui::Event>,
    ) -> Self {
        Confirm {
            id,
            title,
            text,
            yes_button,
            no_button,
            selected_button: Button::No,
            tx,
        }
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect, theme: Arc<dyn Theme>) {
        let [description_area, buttons_area, _] = Layout::vertical([
            Constraint::Min(2),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .areas(area);

        let [_, yes_button_area, no_button_area, _] = Layout::horizontal([
            Constraint::Length(2),
            Constraint::Max((self.yes_button.len() + 2) as u16),
            Constraint::Max((self.no_button.len() + 2) as u16),
            Constraint::Length(2),
        ])
        .areas(buttons_area);
        let description = Paragraph::new(self.text.as_str())
            .block(Block::new().padding(Padding::new(2, 2, 2, 2)))
            .style(theme.foreground_style());
        let block = Block::bordered()
            .title(self.title.as_str())
            .style(theme.popup_style());

        f.render_widget(Clear, area);
        f.render_widget(block, area);
        f.render_widget(description, description_area);
        f.render_widget(
            Self::button(
                self.yes_button.as_str(),
                matches!(self.selected_button, Button::Yes),
                theme.clone(),
            ),
            yes_button_area,
        );
        f.render_widget(
            Self::button(
                self.no_button.as_str(),
                matches!(self.selected_button, Button::No),
                theme.clone(),
            ),
            no_button_area,
        );
    }

    pub fn handle_event(&mut self, e: &Event) -> Result<()> {
        if let Event::Key(key) = e {
            match key {
                KeyEvent {
                    code: KeyCode::Left,
                    ..
                }
                | KeyEvent {
                    code: KeyCode::Right,
                    ..
                } => {
                    self.selected_button = match self.selected_button {
                        Button::Yes => Button::No,
                        Button::No => Button::Yes,
                    };
                    Ok(())
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } => match self.selected_button {
                    Button::Yes => self
                        .tx
                        .send(ui::Event::ClickConfirmYes(self.id.clone()))
                        .map_err(anyhow::Error::from),
                    Button::No => self
                        .tx
                        .send(ui::Event::ClickConfirmNo)
                        .map_err(anyhow::Error::from),
                },
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    fn button(text: &str, selected: bool, theme: Arc<dyn Theme>) -> Paragraph {
        Paragraph::new(format!(" {} ", text)).style(if selected {
            theme.selected_button_style()
        } else {
            theme.button_style()
        })
    }
}
