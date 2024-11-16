use strum::Display;

use crate::models::{FileResult, Line, SearchCondition, SearchResult};
use crate::ui;

type ConfirmId = String;
type SearchConditionCount = usize;
type Inc = usize;

#[derive(Display)]
pub enum Event {
    ChangeResultLine(FileResult, Line),
    SearchFinished(SearchResult),
    ReplaceFinished,
    Progress(Inc),
    StartFileSearch(SearchCondition),
    StartResultSearch(SearchCondition, usize),
    StartReplace,
    SelectCondition(SearchCondition),
    SelectResultLine(FileResult, Line),
    ReplaceSelectLine(FileResult, Line),
    DeleteSearchCondition(SearchConditionCount),
    ShowMessage(Option<ui::Status>),
    Error,
    ClickConfirmYes(ConfirmId),
    ClickConfirmNo,
}
