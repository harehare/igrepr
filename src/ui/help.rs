use itertools::concat;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct Help {}

impl Help {
    pub fn draw(&self, f: &mut Frame, area: Rect) {
        let line = Line::from(concat(vec![
            vec![Span::styled(
                " Tip: ",
                Style::default().add_modifier(Modifier::BOLD),
            )],
            Self::shortcut("Tab", "select command."),
            Self::shortcut_with_modifier("Ctrl", "c", "quit."),
            Self::shortcut_with_modifier("Ctrl", "e", "replace all."),
            Self::shortcut_with_modifier("Ctrl", "n", "copy result."),
            Self::shortcut_with_modifier("Ctrl", "r", "replace on selected rows."),
            Self::shortcut_with_modifier("Ctrl", "v", "show file preview."),
            Self::shortcut_with_modifier("Ctrl", "y", "copy command."),
        ]));

        f.render_widget(Paragraph::new(line), area);
    }

    fn shortcut<'a>(key: &'a str, description: &str) -> Vec<Span<'a>> {
        vec![
            Span::styled(key, Style::default().fg(Color::Green)),
            Span::raw(format!(" => {} ", description)),
        ]
    }

    fn shortcut_with_modifier<'a>(
        shortcut: &'a str,
        key: &'a str,
        description: &str,
    ) -> Vec<Span<'a>> {
        vec![
            Span::styled(
                shortcut,
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" + <", Style::default().fg(Color::Gray)),
            Span::styled(key, Style::default().fg(Color::Green)),
            Span::styled(">", Style::default().fg(Color::Gray)),
            Span::raw(format!(" => {} ", description)),
        ]
    }
}
