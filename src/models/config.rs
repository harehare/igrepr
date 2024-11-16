#[derive(Clone, Debug, Default)]
pub struct SearchConfig {
    pub after_context: Option<usize>,
    pub before_context: Option<usize>,
    pub context_separator: String,
    pub exclude_path: Option<String>,
    pub hide_help: bool,
    pub no_git_exclude: bool,
    pub no_git_ignore: bool,
    pub no_file_name: bool,
    pub no_line_no: bool,
    pub hidden: bool,
    pub max_depth: Option<usize>,
    pub vimgrep: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SearchResultConfig {
    pub after_context: Option<usize>,
    pub before_context: Option<usize>,
    pub context_separator: String,
    pub no_file_name: bool,
    pub no_line_no: bool,
    pub vimgrep: bool,
}

impl SearchConfig {
    pub fn to_search_result_config(&self) -> SearchResultConfig {
        SearchResultConfig {
            after_context: self.after_context,
            before_context: self.before_context,
            context_separator: self.context_separator.clone(),
            no_file_name: self.no_file_name,
            no_line_no: self.no_line_no,
            vimgrep: self.vimgrep,
        }
    }
}
