use crate::{
    app::App,
    models::{Editor, EditorCommand, SearchCondition, SearchConfig},
    parser::parse,
    tui,
    ui::{self, CharIcon, Dark, FontIcon},
};
use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};
use indicatif::ProgressBar;
use std::{
    env,
    io::{self, BufWriter, Read, Write},
    sync::{mpsc, Arc},
};
use strum::Display;

#[derive(Clone, Display, ValueEnum)]
enum Theme {
    Dark,
    Light,
}

#[derive(Parser)]
#[command(name = "igr")]
#[command(author = "Takahiro Sato. <harehare1110@gmail.com>")]
#[command(version = "0.1.0")]
#[command(
    about = "igr - Interactive Grep Result",
    long_about = None
)]
pub struct Cli {
    /// Show lines before each match.
    #[arg(short = 'A', long)]
    after_context: Option<usize>,

    /// Show lines before each match.
    #[arg(short = 'B', long)]
    before_context: Option<usize>,

    /// Show lines before and after each match.
    #[arg(short = 'C', long)]
    context: Option<usize>,

    /// Custom command used to open selected line. e.g.: --custom_command "code -g {file_path}:{line_no}
    #[arg(long, env = "IGR_CUSTOM_COMMAND")]
    custom_command: Option<String>,

    /// The string used to separate
    #[arg(long, default_value_t = String::from("--"))]
    context_separator: String,

    /// Only print the count of individual match lines for each file
    #[arg(short, long)]
    count: bool,

    /// Only print the count of individual matches for each file
    #[arg(long)]
    count_matches: bool,

    /// Disable tui.
    #[arg(short, long)]
    disable_tui: bool,

    /// Text editor used to open selected line.
    #[arg(long, default_value_t = Editor::Vim )]
    editor: Editor,

    /// Search hidden files and directory.
    #[arg(short = '.', long)]
    hidden: bool,

    /// Hide Help.
    #[arg(long)]
    hide_help: bool,

    /// If specified, it excludes files or directories matching the given filename pattern from the search.
    #[arg(long, env = "IGR_EXCLUDE_PATH")]
    exclude_path: Option<String>,

    /// The maximum depth to recurse.
    #[arg(long)]
    max_depth: Option<usize>,

    /// Don't respect .gitignore files.
    #[arg(long)]
    no_git_ignore: bool,

    /// Never print the file path with the matched lines.
    #[arg(short = 'N', long)]
    no_file_name: bool,

    /// Never print the line number with the matched lines.
    #[arg(long)]
    no_line_no: bool,

    /// Not colored the output results.
    #[arg(long)]
    no_color: bool,

    /// Not display icons.
    #[arg(long)]
    no_icon: bool,

    /// Perform replacements if disable_tui is true.
    #[arg(short, long)]
    replace: bool,

    /// Number of grep worker threads to use.
    #[arg(long)]
    threads: Option<usize>,

    /// Specify a theme.
    #[arg(long, value_enum, default_value_t = Theme::Dark)]
    theme: Theme,

    /// Do not output matched lines. instead, exit with status 0 when there is a match and with non-zero status when there isn’t.
    #[arg(short, long)]
    quiet: bool,

    /// Specifies whether all matched results are returned, including row and column numbers.
    #[arg(long)]
    vimgrep: bool,

    /// Searches for specified files and directories
    #[arg(short, long)]
    path: Option<Vec<String>>,

    query: Option<String>,
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        if self.no_color {
            env::set_var("NO_COLOR", "true")
        }

        if self.threads.is_some() {
            env::set_var(
                "RAYON_NUM_THREADS",
                self.threads.map(|n| n.to_string()).unwrap(),
            );
        }

        let path_list = match &self.path {
            Some(p) => p.clone(),
            None => vec![".".to_string()],
        };

        let stdin = if self.is_cli() {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).ok().map(|_| buffer)
        } else {
            None
        };

        let has_stdin = stdin.is_some();
        let conditions: Result<Vec<SearchCondition>> =
            self.query.as_ref().map(|s| parse(&s)).unwrap_or(Ok(vec![]));
        let mut app = match conditions {
            Ok(c) => App::new(
                SearchConfig {
                    after_context: self.after_context.or(self.context),
                    before_context: self.before_context.or(self.context),
                    context_separator: self.context_separator.clone(),
                    exclude_path: self.exclude_path.clone(),
                    hide_help: self.hide_help,
                    no_git_exclude: self.no_git_ignore,
                    no_git_ignore: self.no_git_ignore,
                    no_file_name: self.no_file_name,
                    no_line_no: self.no_line_no,
                    hidden: self.hidden,
                    max_depth: self.max_depth,
                    vimgrep: self.vimgrep,
                },
                path_list,
                c,
                Arc::new(Dark),
                if self.no_icon {
                    Arc::new(CharIcon)
                } else {
                    Arc::new(FontIcon)
                },
                EditorCommand::new(self.editor.clone()),
                stdin,
            ),
            Err(e) => {
                let mut app = App::new(
                    SearchConfig {
                        after_context: self.after_context.or(self.context),
                        before_context: self.before_context.or(self.context),
                        context_separator: self.context_separator.clone(),
                        hide_help: self.hide_help,
                        exclude_path: self.exclude_path.clone(),
                        no_git_exclude: self.no_git_ignore,
                        no_git_ignore: self.no_git_ignore,
                        no_file_name: self.no_file_name,
                        no_line_no: self.no_line_no,
                        hidden: self.hidden,
                        max_depth: self.max_depth,
                        vimgrep: self.vimgrep,
                    },
                    path_list,
                    Vec::new(),
                    Arc::new(Dark),
                    if self.no_icon {
                        Arc::new(CharIcon)
                    } else {
                        Arc::new(FontIcon)
                    },
                    EditorCommand::new(self.editor.clone()),
                    stdin,
                );

                app.set_error(e.to_string());
                app
            }
        };

        if self.is_cli() || has_stdin {
            let result = app.search_sync()?;
            let stdout = io::stdout();
            let handle = stdout.lock();
            let mut writer = BufWriter::new(handle);

            if self.replace {
                if result.files.is_empty() {
                    return Err(anyhow!("No match found"));
                } else if self.quiet {
                    let (tx, _) = mpsc::channel();
                    let handle = std::thread::spawn(move || result.reflect(tx));

                    handle.join().unwrap()?;
                    return Ok(());
                } else {
                    let pb = ProgressBar::new(result.stat().match_count as u64);
                    let (tx, rx) = mpsc::channel();
                    let handle = std::thread::spawn(move || result.reflect(tx));

                    while !handle.is_finished() {
                        if let Ok(ui::Event::Progress(inc)) = rx.try_recv() {
                            pb.inc(inc as u64);
                        }
                    }
                    handle.join().unwrap()?;
                    pb.finish_with_message("done");
                    return Ok(());
                }
            } else if self.quiet {
                if result.files.is_empty() {
                    return Err(anyhow!("No match found"));
                } else {
                    return Ok(());
                }
            } else if self.vimgrep {
                writer.write_all(
                    result
                        .files
                        .into_iter()
                        .map(|f| f.display_vimgrep().to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                        .to_string()
                        .as_bytes(),
                )?;
            } else if self.count {
                writer.write_all(
                    format!(
                        "{}\n",
                        result.files.into_iter().fold(0, |sum, a| {
                            sum + a.lines.into_iter().filter(|l| l.is_line()).count()
                        })
                    )
                    .as_bytes(),
                )?;
            } else if self.count_matches {
                writer.write_all(
                    format!(
                        "{}\n",
                        result.files.into_iter().fold(0, |sum, a| {
                            sum + a
                                .lines
                                .into_iter()
                                .flat_map(|l| l.line().map(|l| l.count_matches()))
                                .sum::<usize>()
                        })
                    )
                    .as_bytes(),
                )?;
            } else {
                writer.write_all(result.to_string().as_bytes())?;
            }

            writer.flush()?;
            return Ok(());
        }

        let mut terminal = tui::init()?;
        let app_result = app.run(&mut terminal);
        tui::restore(terminal)?;
        app_result.map_err(anyhow::Error::from)
    }

    fn is_cli(&self) -> bool {
        self.disable_tui || self.replace || self.count || self.count_matches || self.quiet
    }
}
