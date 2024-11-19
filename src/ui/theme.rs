use ratatui::style::{Color, Modifier, Style};

pub trait Theme {
    fn foreground_style(&self) -> Style;
    fn file_style(&self) -> Style;
    fn line_style(&self) -> Style;
    fn line_no_style(&self) -> Style;
    fn border_style(&self) -> Style;
    fn disabled_style(&self) -> Style;
    fn highlight_style(&self) -> Style;
    fn popup_style(&self) -> Style;
    fn status_style(&self) -> Style;
    fn info_style(&self) -> Style;
    fn error_style(&self) -> Style;
    fn warn_style(&self) -> Style;
    fn count_style(&self) -> Style;
    fn match_style(&self) -> Style;
    fn filter_style(&self) -> Style;
    fn transform_style(&self) -> Style;
    fn button_style(&self) -> Style;
    fn selected_button_style(&self) -> Style;
    fn progressbar_style(&self) -> Style;
}

pub struct Dark;

impl Theme for Dark {
    fn foreground_style(&self) -> Style {
        Style::default().fg(Color::White)
    }

    fn file_style(&self) -> Style {
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD)
    }

    fn line_style(&self) -> Style {
        Style::default().fg(Color::White)
    }

    fn line_no_style(&self) -> Style {
        Style::default()
            .fg(Color::LightCyan)
            .add_modifier(Modifier::BOLD)
    }

    fn border_style(&self) -> Style {
        Style::default().fg(Color::White)
    }

    fn disabled_style(&self) -> Style {
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::CROSSED_OUT)
    }

    fn highlight_style(&self) -> Style {
        Style::default().bg(Color::Rgb(50, 50, 50))
    }

    fn error_style(&self) -> Style {
        Style::default().bg(Color::Red).add_modifier(Modifier::BOLD)
    }

    fn popup_style(&self) -> Style {
        Style::default()
    }

    fn status_style(&self) -> Style {
        Style::default().bg(Color::DarkGray)
    }

    fn info_style(&self) -> Style {
        Style::default()
            .bg(Color::Cyan)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    }

    fn warn_style(&self) -> Style {
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    }

    fn count_style(&self) -> Style {
        Style::default().fg(Color::Green)
    }

    fn match_style(&self) -> Style {
        Style::default().fg(Color::Yellow)
    }

    fn filter_style(&self) -> Style {
        Style::default().fg(Color::Cyan)
    }

    fn transform_style(&self) -> Style {
        Style::default().fg(Color::Green)
    }

    fn button_style(&self) -> Style {
        Style::default().fg(Color::White)
    }

    fn selected_button_style(&self) -> Style {
        Style::default().fg(Color::Black).bg(Color::Yellow)
    }

    fn progressbar_style(&self) -> Style {
        Style::default().fg(Color::Yellow).bg(Color::Gray)
    }
}

pub struct Light;

impl Theme for Light {
    fn foreground_style(&self) -> Style {
        Style::default().fg(Color::Black)
    }

    fn file_style(&self) -> Style {
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD)
    }

    fn line_style(&self) -> Style {
        Style::default().fg(Color::Black)
    }

    fn line_no_style(&self) -> Style {
        Style::default()
            .fg(Color::LightCyan)
            .add_modifier(Modifier::BOLD)
    }

    fn border_style(&self) -> Style {
        Style::default().fg(Color::Black)
    }

    fn disabled_style(&self) -> Style {
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::UNDERLINED)
    }

    fn highlight_style(&self) -> Style {
        Style::default().bg(Color::Rgb(200, 200, 200))
    }

    fn error_style(&self) -> Style {
        Style::default().bg(Color::Red).add_modifier(Modifier::BOLD)
    }

    fn popup_style(&self) -> Style {
        Style::default()
    }

    fn status_style(&self) -> Style {
        Style::default().bg(Color::Gray)
    }

    fn info_style(&self) -> Style {
        Style::default()
            .bg(Color::Cyan)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    }

    fn warn_style(&self) -> Style {
        Style::default()
            .bg(Color::Yellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    }

    fn count_style(&self) -> Style {
        Style::default().fg(Color::Green)
    }

    fn match_style(&self) -> Style {
        Style::default().fg(Color::Yellow)
    }

    fn filter_style(&self) -> Style {
        Style::default().fg(Color::Cyan)
    }

    fn transform_style(&self) -> Style {
        Style::default().fg(Color::Green)
    }

    fn button_style(&self) -> Style {
        Style::default().fg(Color::Black)
    }

    fn selected_button_style(&self) -> Style {
        Style::default().fg(Color::White).bg(Color::Yellow)
    }
    fn progressbar_style(&self) -> Style {
        Style::default().fg(Color::Yellow).bg(Color::LightYellow)
    }
}
