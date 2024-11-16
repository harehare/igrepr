use crate::models;
use anyhow::Result;
use itertools::Itertools;
use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};
use std::ops::Range;
use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings,
};
use syntect_tui::into_span;

#[derive(Clone)]
pub struct FilePreview {
    file_preview: models::FilePreview,
}

impl FilePreview {
    pub fn new(file_path: String) -> Result<Self> {
        models::FilePreview::new(file_path).map(|file_preview| FilePreview { file_preview })
    }

    pub fn is_same_file(&self, file_path: String) -> bool {
        self.file_preview.is_same_file(file_path)
    }

    pub fn draw(&self, f: &mut Frame, area: Rect, line_no: usize) {
        let start = line_no
            .checked_sub(area.height as usize / 2)
            .unwrap_or_default();
        let end = start + area.height as usize;
        let text = self.file_preview.lines(Range { start, end });

        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let syntax = ps.find_syntax_by_extension("rs").unwrap();
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

        let lines = LinesWithEndings::from(text.as_str())
            .map(|line| {
                let line_spans: Vec<Span> = h
                    .highlight_line(line, &ps)
                    .unwrap()
                    .into_iter()
                    .filter_map(|segment| into_span(segment).ok())
                    .collect_vec();
                Line::from(line_spans)
            })
            .collect_vec();
        let text = Paragraph::new(lines).block(Block::bordered().title("File Preview"));

        f.render_widget(text, area);
    }
}
