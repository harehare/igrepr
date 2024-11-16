use super::Line;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileResult {
    pub file_path: String,
    pub lines: Vec<LineResult>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LineResult {
    Line(Line),
    Separator,
}

pub struct FileResultVimGrep {
    pub file_path: String,
    pub lines: Vec<LineResult>,
}

impl Display for FileResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if !self.file_path.is_empty() {
            writeln!(f, "{}", self.file_path)?;
        }

        for line in &self.lines {
            match line {
                LineResult::Line(line) => writeln!(f, "{}", line)?,
                LineResult::Separator => writeln!(f)?,
            }
        }
        Ok(())
    }
}

impl Display for FileResultVimGrep {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for line in &self.lines {
            match line {
                LineResult::Line(line) => {
                    for m in line.matches() {
                        if m.is_found() {
                            writeln!(
                                f,
                                "{}:{}:{}:{}",
                                self.file_path,
                                line.line_no,
                                m.range().start,
                                line
                            )?;
                        }
                    }
                }
                LineResult::Separator => writeln!(f)?,
            }
        }
        Ok(())
    }
}

impl FileResult {
    pub fn display_vimgrep(&self) -> FileResultVimGrep {
        FileResultVimGrep {
            file_path: self.file_path.clone(),
            lines: self.lines.clone(),
        }
    }

    pub fn contains_transformed(&self) -> bool {
        self.lines.iter().any(|line| match line {
            LineResult::Line(line) => line.contains_transformed(),
            LineResult::Separator => false,
        })
    }
}

impl LineResult {
    pub fn is_line(&self) -> bool {
        matches!(self, LineResult::Line(_))
    }

    pub fn line(&self) -> Option<Line> {
        match self {
            LineResult::Line(line) => Some(line.clone()),
            LineResult::Separator => None,
        }
    }
}
