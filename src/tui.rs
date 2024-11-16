use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::*,
};
use ratatui::prelude::*;
use std::io::{self, stdout, Stdout};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> io::Result<Tui> {
    let mut output = stdout();
    enable_raw_mode()?;
    execute!(output, EnterAlternateScreen, EnableMouseCapture)?;
    Terminal::new(CrosstermBackend::new(output))
}

pub fn restore(mut terminal: Tui) -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
