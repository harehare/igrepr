use crate::{
    filter::{self},
    matcher::{self},
    parser, transform,
};
use anyhow::{anyhow, Result};
use std::{
    fmt::{self, Display, Formatter},
    ops::Range,
    str::FromStr,
    sync::Arc,
};
use strum_macros::EnumIter;

use super::{value::Op, Value};

pub const NUMBER: &str = "number";
pub const CONTAINS: &str = "contains";
pub const CONSTANT: &str = "constant";
pub const IGNORE_CASE: &str = "ignore_case";
pub const WHOLE_WORD: &str = "whole_word";
pub const REGEX: &str = "regex";
pub const STARTS_WITH: &str = "starts_with";
pub const ENDS_WITH: &str = "ends_with";
pub const INVERT_MATCH: &str = "invert_match";
pub const INVERT_MATCH_REGEX: &str = "invert_match_regex";
pub const LINE_REGEX: &str = "line.regex";
pub const LINE_STARTS_WITH: &str = "line.starts_with";
pub const LINE_ENDS_WITH: &str = "line.ends_with";
pub const LINE_INVERT_MATCH: &str = "line.invert_match";
pub const LINE_INVERT_MATCH_REGEX: &str = "line.invert_match_regex";
pub const LINE_LENGTH: &str = "line.length";
pub const LINE_BYTE_LENGTH: &str = "line.bytelength";
pub const LINE_CONTAINS: &str = "line.contains";
pub const REPLACE: &str = "replace";
pub const INSERT: &str = "insert";
pub const DELETE: &str = "delete";
pub const CAMEL_CASE: &str = "camel_case";
pub const KEBAB_CASE: &str = "kebab_case";
pub const SNAKE_CASE: &str = "snake_case";
pub const TRIM_END: &str = "trim_end";
pub const TRIM_START: &str = "trim_start";
pub const TRIM: &str = "trim";
pub const UPDATE: &str = "update";
pub const UPPER_CASE: &str = "upper_case";
pub const LOWER_CASE: &str = "lower_case";
pub const UPPER_CAMEL_CASE: &str = "upper_camel_case";
pub const UPPER_KEBAB_CASE: &str = "upper_kebab_case";
pub const UPPER_SNAKE_CASE: &str = "upper_snake_case";

#[derive(Clone, Debug, PartialEq, EnumIter)]
#[strum(serialize_all = "snake_case")]
pub enum SearchCondition {
    // matcher
    Exact(String),
    IgnoreCase(String),
    Number(Option<Op>),
    WholeWord(String),
    Regex(String),
    // filter
    Contains(String),
    StartsWith(String),
    EndsWith(String),
    InvertMatch(String),
    InvertMatchRegex(String),
    // line filter
    LineContains(String),
    LineRegex(String),
    LineStartsWith(String),
    LineEndsWith(String),
    LineInvertMatch(String),
    LineInvertMatchRegex(String),
    LineLength(Option<Op>),
    LineByteLength(Option<Op>),
    // transform
    Replace(String, String),
    Insert(usize, String),
    Delete(usize, usize),
    Update(String),
    CamelCase,
    KebabCase,
    SnakeCase,
    TrimEnd,
    TrimStart,
    Trim,
    Constant,
    LowerCase,
    UpperCase,
    UpperCamelCase,
    UpperKebabCase,
    UpperSnakeCase,
}

impl FromStr for SearchCondition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parser::any_condition(s) {
            Ok((_, c)) => c,
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}

impl Display for SearchCondition {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            SearchCondition::Exact(s) => s.to_string(),
            SearchCondition::Number(Some(Op::Eq(v))) => format!("{}() == {}", NUMBER, v),
            SearchCondition::Number(Some(Op::Ne(v))) => format!("{}() != {}", NUMBER, v),
            SearchCondition::Number(Some(Op::Gt(v))) => format!("{}() > {}", NUMBER, v),
            SearchCondition::Number(Some(Op::Gte(v))) => format!("{}() >= {}", NUMBER, v),
            SearchCondition::Number(Some(Op::Lt(v))) => format!("{}() < {}", NUMBER, v),
            SearchCondition::Number(Some(Op::Lte(v))) => format!("{}() <= {}", NUMBER, v),
            SearchCondition::Number(_) => format!("{}()", NUMBER),
            SearchCondition::IgnoreCase(s) => format!("{}({})", IGNORE_CASE, &s),
            SearchCondition::WholeWord(s) => format!("{}({})", WHOLE_WORD, &s),
            SearchCondition::Regex(s) => format!("{}({})", REGEX, &s),
            SearchCondition::Contains(s) => format!("{}({})", CONTAINS, &s),
            SearchCondition::StartsWith(s) => format!("{}({})", STARTS_WITH, &s),
            SearchCondition::EndsWith(s) => format!("{}({})", ENDS_WITH, &s),
            SearchCondition::InvertMatch(s) => format!("{}({})", INVERT_MATCH, &s),
            SearchCondition::InvertMatchRegex(s) => format!("{}({})", INVERT_MATCH_REGEX, &s),
            SearchCondition::LineContains(s) => format!("{}({})", LINE_CONTAINS, &s),
            SearchCondition::LineRegex(s) => format!("{}({})", LINE_REGEX, &s),
            SearchCondition::LineStartsWith(s) => format!("{}({})", LINE_STARTS_WITH, &s),
            SearchCondition::LineEndsWith(s) => format!("{}({})", LINE_ENDS_WITH, &s),
            SearchCondition::LineInvertMatch(s) => format!("{}({})", LINE_INVERT_MATCH, &s),
            SearchCondition::LineInvertMatchRegex(s) => {
                format!("{}({})", LINE_INVERT_MATCH_REGEX, &s)
            }

            SearchCondition::LineLength(Some(Op::Eq(v))) => {
                format!("{}() == {}", LINE_LENGTH, v)
            }
            SearchCondition::LineLength(Some(Op::Ne(v))) => {
                format!("{}() != {}", LINE_LENGTH, v)
            }
            SearchCondition::LineLength(Some(Op::Gt(v))) => format!("{}() > {}", LINE_LENGTH, v),
            SearchCondition::LineLength(Some(Op::Gte(v))) => format!("{}() >= {}", LINE_LENGTH, v),
            SearchCondition::LineLength(Some(Op::Lt(v))) => {
                format!("{}() < {}", LINE_LENGTH, v)
            }
            SearchCondition::LineLength(Some(Op::Lte(v))) => format!("{}() <= {}", LINE_LENGTH, v),
            SearchCondition::LineLength(_) => {
                format!("{}()", LINE_LENGTH)
            }

            SearchCondition::LineByteLength(Some(Op::Eq(v))) => {
                format!("{}() == {}", LINE_BYTE_LENGTH, v)
            }
            SearchCondition::LineByteLength(Some(Op::Ne(v))) => {
                format!("{}() != {}", LINE_BYTE_LENGTH, v)
            }
            SearchCondition::LineByteLength(Some(Op::Gt(v))) => {
                format!("{}() > {}", LINE_BYTE_LENGTH, v)
            }
            SearchCondition::LineByteLength(Some(Op::Gte(v))) => {
                format!("{}() >= {}", LINE_BYTE_LENGTH, v)
            }
            SearchCondition::LineByteLength(Some(Op::Lt(v))) => {
                format!("{}() < {}", LINE_BYTE_LENGTH, v)
            }
            SearchCondition::LineByteLength(Some(Op::Lte(v))) => {
                format!("{}() <= {}", LINE_BYTE_LENGTH, v)
            }
            SearchCondition::LineByteLength(_) => {
                format!("{}()", LINE_BYTE_LENGTH)
            }

            SearchCondition::Replace(f, t) => format!("{}({}, {})", REPLACE, &f, &t),
            SearchCondition::Insert(i, v) => format!("{}({}, {})", INSERT, &i, &v),
            SearchCondition::Delete(s, e) => format!("{}({}, {})", DELETE, &s, &e),
            SearchCondition::Update(v) => format!("{}({})", UPDATE, &v),
            SearchCondition::CamelCase => format!("{}()", CAMEL_CASE),
            SearchCondition::KebabCase => format!("{}()", KEBAB_CASE),
            SearchCondition::SnakeCase => format!("{}()", SNAKE_CASE),
            SearchCondition::TrimEnd => format!("{}()", TRIM_END),
            SearchCondition::TrimStart => format!("{}()", TRIM_START),
            SearchCondition::Trim => format!("{}()", TRIM),
            SearchCondition::Constant => format!("{}()", CONSTANT),
            SearchCondition::UpperCase => format!("{}()", UPPER_CASE),
            SearchCondition::LowerCase => format!("{}()", LOWER_CASE),
            SearchCondition::UpperCamelCase => format!("{}()", UPPER_CAMEL_CASE),
            SearchCondition::UpperKebabCase => format!("{}()", UPPER_KEBAB_CASE),
            SearchCondition::UpperSnakeCase => format!("{}()", UPPER_SNAKE_CASE),
        };

        write!(f, "{}", s)
    }
}

impl SearchCondition {
    pub fn new_regex(s: &str) -> Result<SearchCondition> {
        matcher::Regex::new(s.to_string()).map(|_| SearchCondition::Regex(s.to_string()))
    }

    pub fn new_line_match_regex(s: &str) -> Result<SearchCondition> {
        filter::Regex::new(s.to_string()).map(|_| SearchCondition::LineRegex(s.to_string()))
    }

    pub fn new_invert_match_regex(s: &str) -> Result<SearchCondition> {
        filter::InvertMatchRegex::new(s.to_string())
            .map(|_| SearchCondition::InvertMatchRegex(s.to_string()))
    }

    pub fn new_line_invert_match_regex(s: &str) -> Result<SearchCondition> {
        filter::InvertMatchRegex::new(s.to_string())
            .map(|_| SearchCondition::LineInvertMatchRegex(s.to_string()))
    }

    pub fn is_matcher(&self) -> bool {
        matches!(self, SearchCondition::Exact(_))
            || matches!(self, SearchCondition::Number(_))
            || matches!(self, SearchCondition::IgnoreCase(_))
            || matches!(self, SearchCondition::WholeWord(_))
            || matches!(self, SearchCondition::Regex(_))
    }

    pub fn is_filter(&self) -> bool {
        matches!(self, SearchCondition::StartsWith(_))
            || matches!(self, SearchCondition::EndsWith(_))
            || matches!(self, SearchCondition::InvertMatch(_))
            || matches!(self, SearchCondition::InvertMatchRegex(_))
            || matches!(self, SearchCondition::Contains(_))
    }

    pub fn is_line_filter(&self) -> bool {
        matches!(self, SearchCondition::LineRegex(_))
            || matches!(self, SearchCondition::LineStartsWith(_))
            || matches!(self, SearchCondition::LineEndsWith(_))
            || matches!(self, SearchCondition::LineInvertMatch(_))
            || matches!(self, SearchCondition::LineInvertMatchRegex(_))
            || matches!(self, SearchCondition::LineLength(_))
            || matches!(self, SearchCondition::LineByteLength(_))
            || matches!(self, SearchCondition::LineContains(_))
    }

    pub fn is_transform(&self) -> bool {
        matches!(self, SearchCondition::Replace(_, _))
            || matches!(self, SearchCondition::Insert(_, _))
            || matches!(self, SearchCondition::Delete(_, _))
            || matches!(self, SearchCondition::Update(_))
            || matches!(self, SearchCondition::CamelCase)
            || matches!(self, SearchCondition::KebabCase)
            || matches!(self, SearchCondition::SnakeCase)
            || matches!(self, SearchCondition::Trim)
            || matches!(self, SearchCondition::TrimEnd)
            || matches!(self, SearchCondition::TrimStart)
            || matches!(self, SearchCondition::Constant)
            || matches!(self, SearchCondition::LowerCase)
            || matches!(self, SearchCondition::UpperCase)
            || matches!(self, SearchCondition::UpperCamelCase)
            || matches!(self, SearchCondition::UpperKebabCase)
            || matches!(self, SearchCondition::UpperSnakeCase)
    }

    pub fn has_args(&self) -> bool {
        self.is_matcher()
            || self.is_filter()
            || self.is_line_filter()
            || !matches!(self, SearchCondition::Exact(_))
            || matches!(self, SearchCondition::Replace(_, _))
            || matches!(self, SearchCondition::Insert(_, _))
            || matches!(self, SearchCondition::Delete(_, _))
            || matches!(self, SearchCondition::Update(_))
    }

    pub fn matcher(&self) -> Option<Arc<dyn matcher::Matcher>> {
        match self.clone() {
            SearchCondition::Exact(s) => Some(matcher::Exact::new(s.to_string())),
            SearchCondition::Number(Some(v)) => Some(matcher::Number::new(v)),
            SearchCondition::IgnoreCase(s) => Some(matcher::IgnoreCase::new(s.to_string())),
            SearchCondition::WholeWord(s) => Some(matcher::WholeWord::new(s.to_string())),
            SearchCondition::Regex(s) => Some(matcher::Regex::new(s.to_string()).unwrap()),
            _ => None,
        }
    }

    pub fn match_filter(&self) -> Option<Arc<dyn filter::Filter>> {
        match self {
            SearchCondition::Contains(s) => Some(filter::Contains::new(s.to_string())),
            SearchCondition::StartsWith(s) => Some(filter::StartsWith::new(s.to_string())),
            SearchCondition::EndsWith(s) => Some(filter::EndsWith::new(s.to_string())),
            SearchCondition::InvertMatch(s) => Some(filter::InvertMatch::new(s.to_string())),
            SearchCondition::InvertMatchRegex(s) => {
                Some(filter::InvertMatchRegex::new(s.to_string()).unwrap())
            }
            _ => None,
        }
    }

    pub fn line_filter(&self) -> Option<Arc<dyn filter::Filter>> {
        match self.clone() {
            SearchCondition::LineContains(s) => Some(filter::Contains::new(s.to_string())),
            SearchCondition::LineRegex(s) => Some(filter::Regex::new(s.to_string()).unwrap()),
            SearchCondition::LineStartsWith(s) => Some(filter::StartsWith::new(s.to_string())),
            SearchCondition::LineEndsWith(s) => Some(filter::EndsWith::new(s.to_string())),
            SearchCondition::LineInvertMatch(s) => Some(filter::InvertMatch::new(s.to_string())),
            SearchCondition::LineInvertMatchRegex(s) => {
                Some(filter::InvertMatchRegex::new(s.to_string()).unwrap())
            }
            SearchCondition::LineLength(Some(op)) => Some(filter::Length::new(false, op)),
            SearchCondition::LineByteLength(Some(op)) => Some(filter::Length::new(true, op)),
            _ => None,
        }
    }

    pub fn transform(&self) -> Option<Arc<dyn transform::Transform>> {
        match self {
            SearchCondition::Replace(f, t) => {
                Some(transform::Replace::new(f.to_string(), t.to_string()))
            }
            SearchCondition::Insert(i, v) => Some(transform::Insert::new(*i, v.to_string())),
            SearchCondition::Delete(s, e) => {
                Some(transform::Delete::new(Range { start: *s, end: *e }))
            }
            SearchCondition::Update(v) => Some(transform::Update::new(v.to_string())),
            SearchCondition::CamelCase => Some(transform::CamelCase::new()),
            SearchCondition::KebabCase => Some(transform::KebabCase::new()),
            SearchCondition::SnakeCase => Some(transform::SnakeCase::new()),
            SearchCondition::TrimEnd => Some(transform::TrimEnd::new()),
            SearchCondition::TrimStart => Some(transform::TrimStart::new()),
            SearchCondition::Constant => Some(transform::UpperSnakeCase::new()),
            SearchCondition::UpperCase => Some(transform::UpperCase::new()),
            SearchCondition::LowerCase => Some(transform::LowerCase::new()),
            SearchCondition::UpperCamelCase => Some(transform::UpperCamelCase::new()),
            SearchCondition::UpperKebabCase => Some(transform::UpperKebabCase::new()),
            SearchCondition::UpperSnakeCase => Some(transform::UpperSnakeCase::new()),
            _ => None,
        }
    }

    pub fn value(&self) -> Option<String> {
        match self {
            SearchCondition::Exact(s) => Some(s.to_string()),

            SearchCondition::Number(Some(Op::Eq(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::Number(Some(Op::Ne(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::Number(Some(Op::Gt(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::Number(Some(Op::Gte(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::Number(Some(Op::Lt(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::Number(Some(Op::Lte(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::Number(_) => None,

            SearchCondition::IgnoreCase(s) => Some(s.to_string()),
            SearchCondition::WholeWord(s) => Some(s.to_string()),
            SearchCondition::Regex(s) => Some(s.to_string()),
            SearchCondition::Contains(s) => Some(s.to_string()),
            SearchCondition::StartsWith(s) => Some(s.to_string()),
            SearchCondition::EndsWith(s) => Some(s.to_string()),
            SearchCondition::InvertMatch(s) => Some(s.to_string()),
            SearchCondition::InvertMatchRegex(s) => Some(s.to_string()),
            SearchCondition::LineContains(s) => Some(s.to_string()),
            SearchCondition::LineRegex(s) => Some(s.to_string()),
            SearchCondition::LineStartsWith(s) => Some(s.to_string()),
            SearchCondition::LineEndsWith(s) => Some(s.to_string()),
            SearchCondition::LineInvertMatch(s) => Some(s.to_string()),
            SearchCondition::LineInvertMatchRegex(s) => Some(s.to_string()),

            SearchCondition::LineLength(Some(Op::Eq(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::LineLength(Some(Op::Ne(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::LineLength(Some(Op::Gt(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::LineLength(Some(Op::Gte(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::LineLength(Some(Op::Lt(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::LineLength(Some(Op::Lte(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::LineLength(_) => None,

            SearchCondition::LineByteLength(Some(Op::Eq(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::LineByteLength(Some(Op::Ne(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::LineByteLength(Some(Op::Gt(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::LineByteLength(Some(Op::Gte(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::LineByteLength(Some(Op::Lt(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::LineByteLength(Some(Op::Lte(Value::Num(n)))) => Some(n.to_string()),
            SearchCondition::LineByteLength(_) => None,

            SearchCondition::Replace(f, t) => Some(format!("{}, {}", f, t)),
            SearchCondition::Insert(i, v) => Some(format!("{}, {}", i, v)),
            SearchCondition::Delete(s, e) => Some(format!("{}, {}", s, e)),
            SearchCondition::Update(v) => Some(v.to_owned()),
            SearchCondition::Constant => None,
            SearchCondition::CamelCase => None,
            SearchCondition::KebabCase => None,
            SearchCondition::SnakeCase => None,
            SearchCondition::TrimEnd => None,
            SearchCondition::TrimStart => None,
            SearchCondition::Trim => None,
            SearchCondition::UpperCase => None,
            SearchCondition::LowerCase => None,
            SearchCondition::UpperCamelCase => None,
            SearchCondition::UpperKebabCase => None,
            SearchCondition::UpperSnakeCase => None,
        }
    }

    pub fn with_value(&self, s: String) -> Result<Self> {
        match self {
            SearchCondition::Exact(_) => Ok(SearchCondition::Exact(s)),
            SearchCondition::IgnoreCase(_) => Ok(SearchCondition::IgnoreCase(s.to_string())),
            SearchCondition::WholeWord(_) => Ok(SearchCondition::WholeWord(s.to_string())),
            SearchCondition::Regex(_) => Ok(SearchCondition::Regex(s.to_string())),
            SearchCondition::Contains(_) => Ok(SearchCondition::Contains(s.to_string())),
            SearchCondition::StartsWith(_) => Ok(SearchCondition::StartsWith(s.to_string())),
            SearchCondition::EndsWith(_) => Ok(SearchCondition::EndsWith(s.to_string())),
            SearchCondition::InvertMatch(_) => Ok(SearchCondition::InvertMatch(s.to_string())),
            SearchCondition::InvertMatchRegex(_) => {
                Ok(SearchCondition::InvertMatchRegex(s.to_string()))
            }
            SearchCondition::LineContains(_) => Ok(SearchCondition::LineContains(s.to_string())),
            SearchCondition::LineRegex(_) => Ok(SearchCondition::LineRegex(s.to_string())),
            SearchCondition::LineStartsWith(_) => {
                Ok(SearchCondition::LineStartsWith(s.to_string()))
            }
            SearchCondition::LineEndsWith(_) => Ok(SearchCondition::LineEndsWith(s.to_string())),
            SearchCondition::LineInvertMatch(_) => {
                Ok(SearchCondition::LineInvertMatch(s.to_string()))
            }
            SearchCondition::LineInvertMatchRegex(_) => {
                Ok(SearchCondition::LineInvertMatchRegex(s.to_string()))
            }
            SearchCondition::Replace(_, _) => match s.split(',').collect::<Vec<_>>().as_slice() {
                [f, t] => Ok(SearchCondition::Replace(f.to_string(), t.to_string())),
                _ => Ok(SearchCondition::Replace(s.to_string(), "".to_string())),
            },
            SearchCondition::Insert(_, _) => match s.split(',').collect::<Vec<_>>().as_slice() {
                [i, v] => Ok(SearchCondition::Insert(
                    i.parse::<usize>().unwrap_or_default(),
                    v.to_string(),
                )),
                _ => Ok(SearchCondition::Insert(0, s.to_string())),
            },
            SearchCondition::Update(_) => Ok(SearchCondition::Update(s.to_string())),
            SearchCondition::Delete(_, _) => match s.split(',').collect::<Vec<_>>().as_slice() {
                [f, t] => Ok(SearchCondition::Delete(
                    f.parse::<usize>().unwrap_or_default(),
                    t.parse::<usize>().unwrap_or_default(),
                )),
                _ => Ok(SearchCondition::Delete(0, 0)),
            },
            _ => Ok(self.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        assert_eq!(SearchCondition::CamelCase.to_string(), "camel_case()");
        assert_eq!(
            SearchCondition::IgnoreCase("test".to_string()).to_string(),
            "ignore_case(test)"
        );
    }
}
