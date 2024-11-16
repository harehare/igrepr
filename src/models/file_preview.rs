use std::{fs, ops::Range};

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct FilePreview {
    file_path: String,
    text: String,
}

impl FilePreview {
    pub fn new(file_path: String) -> Result<Self> {
        match fs::read_to_string(file_path.clone()) {
            Ok(s) => Ok(Self {
                file_path,
                text: s.replace('\t', "    "),
            }),
            Err(e) => Err(anyhow!("Failed to read file: {}", e)),
        }
    }

    pub fn lines(&self, range: Range<usize>) -> String {
        self.text
            .lines()
            .skip(range.start)
            .take(range.end - range.start)
            .join("\n")
    }

    pub fn is_same_file(&self, file_path: String) -> bool {
        self.file_path == file_path
    }
}
