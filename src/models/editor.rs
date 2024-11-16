use anyhow::{anyhow, Result};
use clap::ValueEnum;
use serde::Deserialize;
use std::{
    path::Path,
    process::{Command, ExitStatus},
};

#[derive(Deserialize, Clone, Debug, Default)]
struct EditorConfig {
    #[serde(default = "url_open_command")]
    url_open_command: String,
    github_user_name: Option<String>,
    github_repository_name: Option<String>,
    github_branch_name: Option<String>,
}

#[derive(Clone, Default, strum_macros::Display, ValueEnum)]
#[strum(serialize_all = "kebab_case")]
pub enum Editor {
    #[value(skip)]
    Custom(String),
    Github,
    Emacs,
    Intellij,
    Less,
    Neovim,
    Nano,
    #[default]
    Vim,
    VSCode,
}

pub struct EditorCommand {
    config: EditorConfig,
    editor: Editor,
}

fn url_open_command() -> String {
    "open".to_string()
}

impl EditorCommand {
    pub fn new(editor: Editor) -> Self {
        let config = envy::prefixed("IGR_")
            .from_env::<EditorConfig>()
            .unwrap_or_default();

        Self { editor, config }
    }

    pub fn open(&self, file_path: &str, line_no: usize) -> Result<ExitStatus> {
        let command_args = self.args(file_path, line_no)?;

        match command_args.as_slice() {
            [command, rest @ ..] => {
                let mut child = Command::new(command).args(rest).spawn()?;
                child.wait().map_err(|it| anyhow!(it.to_string()))
            }
            _ => Err(anyhow!("Failed to open")),
        }
    }

    fn command(&self) -> &str {
        match &self.editor {
            Editor::Emacs => "emacs",
            Editor::Intellij => "idea",
            Editor::Less => "less",
            Editor::Nano => "nano",
            Editor::Neovim => "nvim",
            Editor::VSCode => "code",
            Editor::Vim => "vim",
            _ => "",
        }
    }

    fn args(&self, file_path: &str, line_no: usize) -> Result<Vec<String>> {
        match &self.editor {
            Editor::Custom(command) => {
                let command = command.replace("$line_no", &line_no.to_string());
                Ok(command.replace("$file_path", &line_no.to_string()))
            }
            Editor::Emacs => Ok(format!("{} -nw +{line_no} {file_path}", self.command())),
            Editor::Intellij => Ok(format!("{} --line {line_no} {file_path}", self.command())),
            Editor::Less | Editor::Nano | Editor::Neovim | Editor::Vim => {
                Ok(format!("{} +{line_no} {file_path}", self.command()))
            }
            Editor::VSCode => Ok(format!("{} -g {file_path}:{line_no}", self.command())),
            Editor::Github => Ok(format!(
                "{} https://github.com/{}/{}/blob/{}/{}#L{line_no}",
                self.config.url_open_command,
                self.config
                    .clone()
                    .github_user_name
                    .ok_or(anyhow!("Failed to get github user name"))?,
                self.config
                    .clone()
                    .github_repository_name
                    .unwrap_or(Self::git_repository_name()?),
                self.config
                    .clone()
                    .github_branch_name
                    .unwrap_or(Self::git_branch_name()?),
                file_path.replace("./", "")
            )),
        }
        .map(|it| {
            it.split_whitespace()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
        })
    }

    fn git_branch_name() -> Result<String> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
            .map_err(anyhow::Error::from)?;

        String::from_utf8(output.stdout)
            .map_err(anyhow::Error::from)
            .map(|it| it.trim().to_string())
    }

    fn git_repository_name() -> Result<String> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output()
            .map_err(anyhow::Error::from)?;

        let repo_path = String::from_utf8(output.stdout).map_err(anyhow::Error::from)?;

        Path::new(repo_path.trim())
            .file_name()
            .ok_or(anyhow!("Failed to get repository name".to_string()))?
            .to_str()
            .ok_or(anyhow!(
                "Failed to convert repository name to string".to_string()
            ))
            .map(|it| it.to_string())
    }
}
