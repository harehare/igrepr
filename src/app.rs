use crate::{
    clipboard::Clipboard,
    models::{
        editor::EditorCommand, search::Search, search_result::SearchResult, SearchCondition,
        SearchConfig,
    },
    tui,
    ui::{self, Confirm, Help, Icon, Input, InputList, SelectCondition, Theme},
};
use anyhow::{anyhow, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    widgets::{Block, Gauge, Padding, Paragraph},
    Frame,
};
use std::{
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};

const CONFIRM_REPLACE_ID: &str = "confirm_replace";

type TotalCount = usize;
type CurrentCount = usize;

#[derive(PartialEq, Eq)]
enum State {
    Exit,
    Idle,
    Searching,
    Processing(TotalCount, CurrentCount),
}

struct Views {
    confirm: Option<ui::Confirm>,
    file_preview: Option<ui::FilePreview>,
    input_list: InputList,
    search_result: Option<ui::SearchResult>,
    select_condition: Option<ui::SelectCondition>,
    status: Option<ui::Status>,
}

pub struct App<'a> {
    config: SearchConfig,
    search: Search,
    search_result: Option<SearchResult>,
    state: State,
    conditions: Vec<SearchCondition>,
    tx: mpsc::Sender<ui::Event>,
    rx: mpsc::Receiver<ui::Event>,
    views: Views,
    theme: Arc<dyn Theme>,
    icon: Arc<dyn Icon<'a>>,
    editor_command: EditorCommand,
}

impl<'a> App<'a> {
    pub fn new(
        config: SearchConfig,
        path_list: Vec<String>,
        conditions: Vec<SearchCondition>,
        theme: Arc<dyn Theme>,
        icon: Arc<dyn Icon<'a>>,
        editor_command: EditorCommand,
        stdin: Option<String>,
    ) -> Self {
        let (tx, rx) = mpsc::channel();

        Self {
            config,
            search: Search::new(path_list.clone(), stdin),
            search_result: None,
            state: State::Idle,
            conditions: conditions.clone(),
            tx: tx.clone(),
            rx,
            views: Views {
                confirm: None,
                file_preview: None,
                input_list: InputList::new(
                    conditions
                        .into_iter()
                        .map(|c| Input::entered(c, tx.clone()))
                        .collect(),
                    tx.clone(),
                ),
                search_result: None,
                select_condition: None,
                status: Some(ui::Status::new(Some(ui::Message::Info(format!(
                    "Directory to search => {}",
                    path_list.join(", ")
                ))))),
            },
            theme,
            icon,
            editor_command,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
        self.search_async(self.conditions.clone());

        while self.state != State::Exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events(terminal);
            self.handle_inputs()?;
        }
        Ok(())
    }

    pub fn set_error(&mut self, message: String) {
        self.views.status = Some(ui::Status::new(Some(ui::Message::Error(message))));
    }

    fn draw(&mut self, f: &mut Frame) {
        let [input_area, list_area, status_area, help_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(f.area().height - if self.config.hide_help { 4 } else { 5 }),
            Constraint::Length(1),
            Constraint::Length(if self.config.hide_help { 0 } else { 1 }),
        ])
        .areas(f.area());

        self.views
            .input_list
            .draw(f, input_area, self.theme.clone(), self.icon.clone());
        match self.state {
            State::Idle => {
                if let Some(result) = self.views.search_result.as_mut() {
                    if let Some(file_preview) = self.views.file_preview.as_mut() {
                        let [list_area, preview_area] = Layout::horizontal([
                            Constraint::Percentage(50),
                            Constraint::Percentage(50),
                        ])
                        .areas(list_area);
                        let line_no = result.selected().map(|(_, l)| l.line_no).unwrap_or(0);
                        file_preview.draw(f, preview_area, line_no);
                        result.draw(f, list_area, self.theme.clone());
                    } else {
                        result.draw(f, list_area, self.theme.clone());
                    }
                } else {
                    let message = Paragraph::new("Please enter search keywords")
                        .style(self.theme.foreround_style())
                        .block(Block::default().padding(Padding::top(2)))
                        .centered();
                    f.render_widget(message, list_area);
                }
            }
            State::Searching => {
                let p = Paragraph::new("Searching...")
                    .block(Block::default().padding(Padding::top(2)))
                    .alignment(Alignment::Center);
                f.render_widget(p, list_area);
            }
            State::Processing(total, current) => {
                let [_, progress_area] =
                    Layout::vertical([Constraint::Length(2), Constraint::Length(2)])
                        .margin(1)
                        .areas(list_area);
                let p = Gauge::default()
                    .block(Block::default().title("Processing..."))
                    .gauge_style(self.theme.progressbar_style())
                    .use_unicode(true)
                    .ratio(current as f64 / total as f64);
                f.render_widget(p, progress_area);
                self.views.status = Some(ui::Status::new(Some(ui::Message::Info(format!(
                    "Processing {}/{}",
                    current, total
                )))));
            }
            State::Exit => (),
        };

        if let Some(select_condition_popup) = self.views.select_condition.as_mut() {
            let popup_area = App::centered_rect(90, 90, f.area());
            select_condition_popup.draw(
                f,
                popup_area,
                self.theme.clone(),
                self.views.input_list.entered_list.is_empty(),
                self.icon.clone(),
            );
        }

        if let Some(confirm) = self.views.confirm.as_mut() {
            let popup_area = App::centered_rect(60, 60, f.area());
            confirm.draw(f, popup_area, self.theme.clone());
        }

        if let Some(s) = self.views.status.as_mut() {
            s.draw(f, status_area, self.theme.clone());
        }

        if !self.config.hide_help {
            Help {}.draw(f, help_area);
        }
    }

    fn handle_events(&mut self, terminal: &mut tui::Tui) {
        if let Ok(event) = self.rx.try_recv() {
            match event {
                ui::Event::ClickConfirmNo => {
                    self.views.confirm = None;
                }
                ui::Event::ClickConfirmYes(confirm_id) if confirm_id == CONFIRM_REPLACE_ID => {
                    self.views.confirm = None;
                    self.views.status = Some(ui::Status::new(Some(ui::Message::Info(
                        "Processing...".to_string(),
                    ))));
                    self.replace_async();
                }
                ui::Event::ClickConfirmYes(_) => (),
                ui::Event::SearchFinished(result) => {
                    self.state = State::Idle;
                    self.search_result = Some(result.clone());
                    self.views.search_result = Some(ui::SearchResult::new(
                        &result.files,
                        self.config.to_search_result_config(),
                        self.tx.clone(),
                    ));
                    self.views.status =
                        Some(ui::Status::new(Some(ui::Message::Stat(result.stat()))));
                }
                ui::Event::Error => {
                    // TODO: handle error
                    self.state = State::Exit;
                }
                ui::Event::DeleteSearchCondition(count) if count > 0 => {
                    self.delete_last_condition()
                }
                ui::Event::DeleteSearchCondition(_) => self.delete_first_condition(),
                ui::Event::StartFileSearch(c) => {
                    self.views.status = Some(ui::Status::new(Some(ui::Message::Info(
                        "Searching...".to_string(),
                    ))));
                    self.search_async(vec![c]);
                }
                ui::Event::StartResultSearch(c, index) => {
                    self.views.status = Some(ui::Status::new(Some(ui::Message::Info(
                        "Searching...".to_string(),
                    ))));
                    self.apply(c, index);
                }
                ui::Event::ReplaceSelectLine(f, l) if self.views.input_list.has_transform() => {
                    if let Some(result) = self.search_result.as_mut() {
                        if result.reflect_on_selected_row(&f, &l).is_ok() {
                            self.views.search_result = Some(ui::SearchResult::new(
                                &result.files,
                                self.config.to_search_result_config(),
                                self.tx.clone(),
                            ));
                            self.views.status =
                                Some(ui::Status::new(Some(ui::Message::Info(format!(
                                    "Replaced {} occurrences in {}",
                                    &l.matches().len(),
                                    f.file_path
                                )))));
                        } else {
                            self.views.status = Some(ui::Status::new(Some(ui::Message::Error(
                                "Replace failed".to_string(),
                            ))));
                        }
                    }
                }
                ui::Event::ReplaceSelectLine(_, _) => (),
                ui::Event::SelectResultLine(f, l) => {
                    match self.editor_command.open(f.file_path.as_str(), l.line_no) {
                        Ok(_) => (),
                        Err(e) => {
                            self.views.status =
                                Some(ui::Status::new(Some(ui::Message::Error(e.to_string()))))
                        }
                    }
                    terminal.clear().unwrap();
                }
                ui::Event::SelectCondition(c) => {
                    self.views.input_list.set_current_condition(c);
                    self.views.select_condition = None;
                }
                ui::Event::ShowMessage(msg) => {
                    self.views.status = msg;
                }
                ui::Event::Progress(inc) => {
                    if let State::Processing(total, current) = self.state {
                        self.state = State::Processing(total, current + inc);
                    }
                }
                ui::Event::ChangeResultLine(f, _) => {
                    if let Some(file_preview) = self.views.file_preview.as_mut() {
                        if !file_preview.is_same_file(f.file_path.clone()) {
                            if let Ok(f) = ui::FilePreview::new(f.file_path) {
                                self.views.file_preview = Some(f);
                            }
                        }
                    }
                }
                ui::Event::StartReplace => {
                    self.views.status = Some(ui::Status::new(Some(ui::Message::Info(
                        "Processing...".to_string(),
                    ))));
                    self.replace_async();
                }
                ui::Event::ReplaceFinished => {
                    self.state = State::Idle;

                    if let Some(result) = self.search_result.as_mut() {
                        let stat = result.stat();
                        self.search_result = None;
                        self.views.search_result = None;
                        self.views.input_list = InputList::new(Vec::new(), self.tx.clone());
                        self.views.status =
                            Some(ui::Status::new(Some(ui::Message::Info(format!(
                                "Replaced {} occurrences across {} files",
                                stat.match_count, stat.file_count
                            )))));
                    }
                }
            }
        }
    }

    fn handle_inputs(&mut self) -> Result<()> {
        let duration = if matches!(self.state, State::Searching) {
            Duration::from_millis(1)
        } else {
            Duration::from_millis(100)
        };

        if event::poll(duration)? {
            if let Ok(Event::Key(key)) = event::read() {
                if let Some(c) = self.views.select_condition.as_mut() {
                    c.handle_event(&Event::Key(key));
                } else if let Some(c) = self.views.confirm.as_mut() {
                    c.handle_event(&Event::Key(key))?;
                } else if matches!(self.state, State::Idle)
                    && !self.views.input_list.handle_event(&Event::Key(key))
                {
                    if let Some(r) = self.views.search_result.as_mut() {
                        r.handle_event(&Event::Key(key));
                    }
                }

                match key {
                    KeyEvent {
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        self.state = State::Exit;
                    }
                    KeyEvent {
                        code: KeyCode::Char('e'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        if !self.views.input_list.has_transform() {
                            return Ok(());
                        }

                        if let Some(result) = self.search_result.as_mut() {
                            let stat = result.stat();

                            self.views.confirm = Some(Confirm::new(
                                CONFIRM_REPLACE_ID.to_string(),
                                "Confirmation".to_string(),
                                format!(
                                    "Replace {} occurrences across {} files?",
                                    stat.match_count, stat.file_count
                                ),
                                "Replace".to_string(),
                                "Cancel".to_string(),
                                self.tx.clone(),
                            ));
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Tab, ..
                    } => {
                        self.views.select_condition = Some(SelectCondition::new(
                            self.views.input_list.input_value(),
                            self.tx.clone(),
                        ));
                    }
                    KeyEvent {
                        code: KeyCode::Char('v'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        if let Some(r) = self.views.search_result.as_mut() {
                            if self.views.file_preview.is_some() {
                                self.views.file_preview = None;
                            } else if let Some(f) = r
                                .selected()
                                .and_then(|(file, _)| ui::FilePreview::new(file.file_path).ok())
                            {
                                self.views.file_preview = Some(f);
                            }
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Char('n'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        if let Some(r) = &self.search_result {
                            Clipboard::copy(r.to_conditions_string())?;
                            self.views.status = Some(ui::Status::new(Some(ui::Message::Info(
                                "copied the query to clipboard".to_string(),
                            ))));
                            self.clear_message();
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Char('y'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => {
                        if let Some(r) = &self.search_result {
                            Clipboard::copy(r.to_string())?;
                            self.views.status = Some(ui::Status::new(Some(ui::Message::Info(
                                "copied the search results to clipboard".to_string(),
                            ))));
                            self.clear_message();
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Esc, ..
                    } => {
                        self.views.select_condition = None;
                        self.views.confirm = None;
                    }
                    _ => (),
                }
            }
        }

        Ok(())
    }

    pub fn search_sync(&mut self) -> Result<SearchResult> {
        let search = self.search.clone();
        let config = self.config.clone();

        match self.conditions.as_slice() {
            [find, ..] if find.is_transform() => Err(anyhow!("Invalid search condition")),
            [cond1, cond2, rest @ ..] => {
                let rest_conditions = rest.to_vec().clone();
                let result = search.search(config, vec![cond1.clone(), cond2.clone()]);
                if rest_conditions.is_empty() {
                    Ok(result)
                } else {
                    Ok(rest_conditions
                        .into_iter()
                        .fold(result, |r, c| r.apply(c, 2)))
                }
            }
            [cond, rest @ ..] => {
                let rest_conditions = rest.to_vec().clone();
                let result = search.search(config, vec![cond.clone()]);
                if rest_conditions.is_empty() {
                    Ok(result)
                } else {
                    Ok(rest_conditions
                        .into_iter()
                        .fold(result, |r, c| r.apply(c, 2)))
                }
            }
            [] => Err(anyhow!("Invalid search condition")),
        }
    }

    fn search_async(&mut self, conditions: Vec<SearchCondition>) {
        let search = self.search.clone();
        let tx = self.tx.clone();
        let config = self.config.clone();

        match conditions.as_slice() {
            [find, ..] if find.is_transform() => {
                self.views.status = Some(ui::Status::new(Some(ui::Message::Error(
                    "Invalid search condition".to_string(),
                ))));
            }
            [cond1, cond2, rest @ ..] => {
                let rest_conditions = rest.to_vec().clone();
                let cond1 = cond1.clone();
                let cond2 = cond2.clone();

                self.state = State::Searching;

                std::thread::spawn(move || {
                    let result = search.search(config, vec![cond1, cond2]);
                    let result = if rest_conditions.is_empty() {
                        result
                    } else {
                        rest_conditions
                            .into_iter()
                            .fold(result, |r, c| r.apply(c, 2))
                    };

                    tx.send(ui::Event::SearchFinished(result)).ok();
                });
            }
            [cond, rest @ ..] => {
                let rest_conditions = rest.to_vec().clone();
                let cond = cond.clone();

                self.state = State::Searching;

                std::thread::spawn(move || {
                    let result = search.search(config, vec![cond]);
                    let result = if rest_conditions.is_empty() {
                        result
                    } else {
                        rest_conditions
                            .into_iter()
                            .fold(result, |r, c| r.apply(c, 2))
                    };

                    tx.send(ui::Event::SearchFinished(result)).ok();
                });
            }
            [] => (),
        }
    }

    fn replace_async(&mut self) {
        let tx = self.tx.clone();

        if let Some(result) = self.search_result.clone() {
            self.state = State::Processing(result.stat().file_count, 0);
            std::thread::spawn(move || {
                std::thread::spawn(move || match result.reflect(tx.clone()) {
                    Ok(_) => tx.send(ui::Event::ReplaceFinished).ok(),
                    Err(e) => tx
                        .send(ui::Event::ShowMessage(Some(ui::Status::new(Some(
                            ui::Message::Error(e.to_string()),
                        )))))
                        .ok(),
                });
            });
        };
    }

    fn apply(&mut self, matcher: SearchCondition, index: usize) {
        let tx = self.tx.clone();

        if let Some(result) = self.search_result.clone() {
            self.state = State::Searching;
            std::thread::spawn(move || {
                tx.send(ui::Event::SearchFinished(result.apply(matcher, index)))
                    .ok();
            });
        };
    }

    fn delete_first_condition(&mut self) {
        self.state = State::Idle;
        self.search_result = None;
        self.views.search_result = None;
    }

    fn delete_last_condition(&mut self) {
        let tx = self.tx.clone();

        if let Some(mut result) = self.search_result.clone() {
            self.state = State::Searching;
            std::thread::spawn(move || {
                tx.send(ui::Event::SearchFinished(result.delete_last_condition()))
                    .ok();
            });
        };
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::vertical([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

        Layout::horizontal([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
    }

    fn clear_message(&mut self) {
        let tx = self.tx.clone();

        if let Some(result) = self.search_result.clone() {
            std::thread::spawn(move || {
                thread::sleep(Duration::from_secs(3));
                tx.send(ui::Event::ShowMessage(Some(ui::Status::new(Some(
                    ui::Message::Stat(result.stat()),
                )))))
                .ok();
            });
        } else {
            std::thread::spawn(move || {
                thread::sleep(Duration::from_secs(3));
                tx.send(ui::Event::ShowMessage(None)).ok();
            });
        };
    }
}
