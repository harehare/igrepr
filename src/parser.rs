use crate::models::search_condition::{
    CAMEL_CASE, CONSTANT, CONTAINS, DELETE, ENDS_WITH, IGNORE_CASE, INSERT, INVERT_MATCH,
    INVERT_MATCH_REGEX, KEBAB_CASE, LINE_BYTE_LENGTH, LINE_CONTAINS, LINE_ENDS_WITH,
    LINE_INVERT_MATCH, LINE_INVERT_MATCH_REGEX, LINE_LENGTH, LINE_REGEX, LINE_STARTS_WITH,
    LOWER_CASE, NUMBER, REGEX, REPLACE, SNAKE_CASE, STARTS_WITH, TRIM, TRIM_END, TRIM_START,
    UPDATE, UPPER_CAMEL_CASE, UPPER_CASE, UPPER_KEBAB_CASE, UPPER_SNAKE_CASE, WHOLE_WORD,
};
use crate::models::value::Op;
use crate::models::{SearchCondition, Value};
use anyhow::Result;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::{eof, rest};
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::{
    bytes::complete::{is_not, tag},
    combinator::{value, verify},
    sequence::tuple,
    IResult,
};

pub fn parse(s: &str) -> Result<Vec<SearchCondition>> {
    match many0(any_condition)(s) {
        Ok(r) => r.1.into_iter().collect(),
        Err(_) => Ok(vec![]),
    }
}

pub fn any_condition(s: &str) -> IResult<&str, Result<SearchCondition>> {
    alt((
        matcher_string,
        any_matcher,
        any_filter,
        any_line_filter,
        any_transform,
        exact,
    ))(s)
}

fn any_matcher(s: &str) -> IResult<&str, Result<SearchCondition>> {
    alt((
        number_eq,
        number_ne,
        number_gt,
        number_gte,
        number_lt,
        number_lte,
        number,
        ignore_case,
        whole_word,
        regex,
    ))(s)
}

fn any_filter(s: &str) -> IResult<&str, Result<SearchCondition>> {
    alt((
        starts_with,
        ends_with,
        invert_match,
        invert_match_regex,
        contains,
    ))(s)
}

fn any_line_filter(s: &str) -> IResult<&str, Result<SearchCondition>> {
    alt((
        line_contains,
        line_regex,
        line_invert_match,
        line_starts_with,
        line_filter_ends_with,
        line_invert_match_regex,
        line_length_eq,
        line_length_ne,
        line_length_gt,
        line_length_gte,
        line_length_lt,
        line_length_lte,
        line_bytelength_eq,
        line_bytelength_ne,
        line_bytelength_gt,
        line_bytelength_gte,
        line_bytelength_lt,
        line_bytelength_lte,
    ))(s)
}

fn any_transform(s: &str) -> IResult<&str, Result<SearchCondition>> {
    alt((
        replace,
        insert,
        update,
        delete,
        camel_case,
        kebab_case,
        snake_case,
        trim_end,
        trim_start,
        trim,
        constant,
        lower_case,
        upper_case,
        upper_camel_case,
        upper_kebab_case,
        upper_snake_case,
    ))(s)
}

fn string(s: &str) -> IResult<&str, &str> {
    verify(is_not(")"), |s: &str| !s.is_empty())(s.trim())
}

fn separator(s: &str) -> IResult<&str, &str> {
    alt((eof, preceded(char('|'), rest)))(s.trim())
}

fn is_valid_env_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn env_name(s: &str) -> IResult<&str, &str> {
    take_while1(is_valid_env_char)(s.trim())
}

fn expr_env(s: &str) -> IResult<&str, Value> {
    tuple((tag("env."), env_name, rest))(s)
        .map(|(_, (_, e, s))| (s.trim(), Value::Env(e.to_string())))
}

fn expr_num_or_string(s: &str) -> IResult<&str, Value> {
    tuple((alt((digit1, string)), rest))(s).map(|(_, (v, s))| {
        (
            s.trim(),
            v.parse()
                .map(Value::Num)
                .unwrap_or_else(|_| Value::Str(v.to_string())),
        )
    })
}

fn val(s: &str) -> IResult<&str, Value> {
    tuple((alt((expr_env, expr_num_or_string)), rest))(s).map(|(_, (v, s))| (s.trim(), v))
}

fn gt(s: &str) -> IResult<&str, Value> {
    tuple((delimited(space0, tag(">"), space0), val, rest))(s).map(|(_, (_, v, s))| (s.trim(), v))
}

fn gte(s: &str) -> IResult<&str, Value> {
    tuple((delimited(space0, tag(">="), space0), val, rest))(s).map(|(_, (_, v, s))| (s.trim(), v))
}

fn lt(s: &str) -> IResult<&str, Value> {
    tuple((delimited(space0, tag("<"), space0), val, rest))(s).map(|(_, (_, v, s))| (s.trim(), v))
}

fn lte(s: &str) -> IResult<&str, Value> {
    tuple((delimited(space0, tag("<="), space0), val, rest))(s).map(|(_, (_, v, s))| (s.trim(), v))
}

fn eq(s: &str) -> IResult<&str, Value> {
    tuple((delimited(space0, tag("=="), space0), val, rest))(s).map(|(_, (_, v, s))| (s.trim(), v))
}

fn ne(s: &str) -> IResult<&str, Value> {
    tuple((delimited(space0, tag("!="), space0), val, rest))(s).map(|(_, (_, v, s))| (s.trim(), v))
}

fn matcher_string(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        delimited(
            tag("'"),
            verify(is_not("'"), |s: &str| !s.is_empty()),
            tag("'"),
        ),
        separator,
    ))(s)
    .map(|(_, (_, matched, s))| {
        (
            s.trim(),
            Ok(SearchCondition::Exact(matched.trim().to_string())),
        )
    })
}

fn ignore_case(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(IGNORE_CASE),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| {
        (
            s.trim(),
            Ok(SearchCondition::IgnoreCase(matched.to_string())),
        )
    })
}

fn exact(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        verify(is_not("|"), |s: &str| !s.is_empty()),
        separator,
    ))(s)
    .map(|(_, (_, matched, s))| {
        (
            s.trim(),
            Ok(SearchCondition::Exact(matched.trim().to_string())),
        )
    })
}

fn number(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        value(SearchCondition::Number(None), tag(NUMBER)),
        tag("()"),
        separator,
    ))(s)
    .map(|(_, (_, m, _, s))| (s.trim(), Ok(m)))
}

fn number_eq(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(NUMBER), tag("()"), eq, separator))(s)
        .map(|(_, (_, _, _, v, s))| (s.trim(), Ok(SearchCondition::Number(Some(Op::Eq(v))))))
}

fn number_ne(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(NUMBER), tag("()"), ne, separator))(s)
        .map(|(_, (_, _, _, v, s))| (s.trim(), Ok(SearchCondition::Number(Some(Op::Ne(v))))))
}

fn number_gt(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(NUMBER), tag("()"), gt, separator))(s)
        .map(|(_, (_, _, _, v, s))| (s.trim(), Ok(SearchCondition::Number(Some(Op::Gt(v))))))
}

fn number_gte(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(NUMBER), tag("()"), gte, separator))(s)
        .map(|(_, (_, _, _, v, s))| (s.trim(), Ok(SearchCondition::Number(Some(Op::Gte(v))))))
}

fn number_lt(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(NUMBER), tag("()"), lt, separator))(s)
        .map(|(_, (_, _, _, v, s))| (s.trim(), Ok(SearchCondition::Number(Some(Op::Lt(v))))))
}

fn number_lte(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(NUMBER), tag("()"), lte, separator))(s)
        .map(|(_, (_, _, _, v, s))| (s.trim(), Ok(SearchCondition::Number(Some(Op::Lte(v))))))
}

fn regex(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(REGEX),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| (s.trim(), SearchCondition::new_regex(matched)))
}

fn whole_word(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(WHOLE_WORD),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| {
        (
            s.trim(),
            Ok(SearchCondition::WholeWord(matched.to_string())),
        )
    })
}

fn starts_with(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(STARTS_WITH),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| {
        (
            s.trim(),
            Ok(SearchCondition::StartsWith(matched.to_string())),
        )
    })
}

fn line_starts_with(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(LINE_STARTS_WITH),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| {
        (
            s.trim(),
            Ok(SearchCondition::LineStartsWith(matched.to_string())),
        )
    })
}

fn contains(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(CONTAINS),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| (s.trim(), Ok(SearchCondition::Contains(matched.to_string()))))
}

fn ends_with(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(ENDS_WITH),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| (s.trim(), Ok(SearchCondition::EndsWith(matched.to_string()))))
}

fn line_filter_ends_with(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(LINE_ENDS_WITH),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| {
        (
            s.trim(),
            Ok(SearchCondition::LineEndsWith(matched.to_string())),
        )
    })
}

fn invert_match(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(INVERT_MATCH),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| {
        (
            s.trim(),
            Ok(SearchCondition::InvertMatch(matched.to_string())),
        )
    })
}

fn line_regex(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(LINE_REGEX),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| (s.trim(), SearchCondition::new_line_match_regex(matched)))
}

fn line_contains(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(LINE_CONTAINS),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| {
        (
            s.trim(),
            Ok(SearchCondition::LineContains(matched.to_string())),
        )
    })
}

fn line_invert_match(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(LINE_INVERT_MATCH),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| {
        (
            s.trim(),
            Ok(SearchCondition::LineInvertMatch(matched.to_string())),
        )
    })
}

fn invert_match_regex(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(INVERT_MATCH_REGEX),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| (s.trim(), SearchCondition::new_invert_match_regex(matched)))
}

fn line_invert_match_regex(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(LINE_INVERT_MATCH_REGEX),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| {
        (
            s.trim(),
            SearchCondition::new_line_invert_match_regex(matched),
        )
    })
}

fn line_length_eq(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(LINE_LENGTH), tag("()"), eq, separator))(s)
        .map(|(_, (_, _, _, v, s))| (s.trim(), Ok(SearchCondition::LineLength(Some(Op::Eq(v))))))
}

fn line_length_ne(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(LINE_LENGTH), tag("()"), ne, separator))(s)
        .map(|(_, (_, _, _, v, s))| (s.trim(), Ok(SearchCondition::LineLength(Some(Op::Ne(v))))))
}

fn line_length_gt(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(LINE_LENGTH), tag("()"), gt, separator))(s)
        .map(|(_, (_, _, _, v, s))| (s.trim(), Ok(SearchCondition::LineLength(Some(Op::Gt(v))))))
}

fn line_length_gte(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(LINE_LENGTH), tag("()"), gte, separator))(s)
        .map(|(_, (_, _, _, v, s))| (s.trim(), Ok(SearchCondition::LineLength(Some(Op::Gte(v))))))
}

fn line_length_lt(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(LINE_LENGTH), tag("()"), lt, separator))(s)
        .map(|(_, (_, _, _, v, s))| (s.trim(), Ok(SearchCondition::LineLength(Some(Op::Lt(v))))))
}

fn line_length_lte(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(LINE_LENGTH), tag("()"), lte, separator))(s)
        .map(|(_, (_, _, _, v, s))| (s.trim(), Ok(SearchCondition::LineLength(Some(Op::Lte(v))))))
}

fn line_bytelength_eq(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(LINE_BYTE_LENGTH), tag("()"), eq, separator))(s).map(
        |(_, (_, _, _, v, s))| {
            (
                s.trim(),
                Ok(SearchCondition::LineByteLength(Some(Op::Eq(v)))),
            )
        },
    )
}

fn line_bytelength_ne(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(LINE_BYTE_LENGTH), tag("()"), ne, separator))(s).map(
        |(_, (_, _, _, v, s))| {
            (
                s.trim(),
                Ok(SearchCondition::LineByteLength(Some(Op::Ne(v)))),
            )
        },
    )
}

fn line_bytelength_gt(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(LINE_BYTE_LENGTH), tag("()"), gt, separator))(s).map(
        |(_, (_, _, _, v, s))| {
            (
                s.trim(),
                Ok(SearchCondition::LineByteLength(Some(Op::Gt(v)))),
            )
        },
    )
}

fn line_bytelength_gte(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(LINE_BYTE_LENGTH), tag("()"), gte, separator))(s).map(
        |(_, (_, _, _, v, s))| {
            (
                s.trim(),
                Ok(SearchCondition::LineByteLength(Some(Op::Gte(v)))),
            )
        },
    )
}

fn line_bytelength_lt(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(LINE_BYTE_LENGTH), tag("()"), lt, separator))(s).map(
        |(_, (_, _, _, v, s))| {
            (
                s.trim(),
                Ok(SearchCondition::LineByteLength(Some(Op::Lt(v)))),
            )
        },
    )
}

fn line_bytelength_lte(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((space0, tag(LINE_BYTE_LENGTH), tag("()"), lte, separator))(s).map(
        |(_, (_, _, _, v, s))| {
            (
                s.trim(),
                Ok(SearchCondition::LineByteLength(Some(Op::Lte(v)))),
            )
        },
    )
}

fn replace(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(REPLACE),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| {
        let (from, to) = match matched.split(',').collect::<Vec<_>>().as_slice() {
            [f, t] => (f.trim().to_string(), t.trim().to_string()),
            _ => (matched.trim().to_string(), "".to_string()),
        };
        (
            s.trim(),
            Ok(SearchCondition::Replace(from.to_string(), to.to_string())),
        )
    })
}

fn update(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(UPDATE),
        delimited(char('('), string, char(')')),
        separator,
    ))(s)
    .map(|(_, (_, _, matched, s))| (s.trim(), Ok(SearchCondition::Update(matched.to_string()))))
}

fn insert(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(INSERT),
        char('('),
        digit1,
        space0,
        tag(","),
        space0,
        string,
        char(')'),
        separator,
    ))(s)
    .map(|(_, (_, _, _, index, _, _, _, value, _, s))| {
        (
            s.trim(),
            Ok(SearchCondition::Insert(
                index.parse().unwrap(),
                value.to_string(),
            )),
        )
    })
}

fn delete(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        tag(DELETE),
        char('('),
        digit1,
        space0,
        tag(","),
        space0,
        digit1,
        char(')'),
        separator,
    ))(s)
    .map(|(_, (_, _, _, start, _, _, _, end, _, s))| {
        (
            s.trim(),
            Ok(SearchCondition::Delete(
                start.parse().unwrap(),
                end.parse().unwrap(),
            )),
        )
    })
}

fn camel_case(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        value(SearchCondition::CamelCase, tag(CAMEL_CASE)),
        tag("()"),
        separator,
    ))(s)
    .map(|(_, (_, m, _, s))| (s.trim(), Ok(m)))
}

fn kebab_case(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        value(SearchCondition::KebabCase, tag(KEBAB_CASE)),
        tag("()"),
        separator,
    ))(s)
    .map(|(_, (_, m, _, s))| (s.trim(), Ok(m)))
}

fn snake_case(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        value(SearchCondition::SnakeCase, tag(SNAKE_CASE)),
        tag("()"),
        separator,
    ))(s)
    .map(|(_, (_, m, _, s))| (s.trim(), Ok(m)))
}

fn trim_end(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        value(SearchCondition::TrimEnd, alt((tag(TRIM_END), tag("te")))),
        tag("()"),
        separator,
    ))(s)
    .map(|(_, (_, m, _, s))| (s.trim(), Ok(m)))
}

fn trim_start(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        value(
            SearchCondition::TrimStart,
            alt((tag(TRIM_START), tag("ts"))),
        ),
        tag("()"),
        separator,
    ))(s)
    .map(|(_, (_, m, _, s))| (s.trim(), Ok(m)))
}

fn trim(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        value(SearchCondition::Trim, tag(TRIM)),
        tag("()"),
        separator,
    ))(s)
    .map(|(_, (_, m, _, s))| (s.trim(), Ok(m)))
}

fn lower_case(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        value(SearchCondition::LowerCase, tag(LOWER_CASE)),
        tag("()"),
        separator,
    ))(s)
    .map(|(_, (_, m, _, s))| (s.trim(), Ok(m)))
}

fn constant(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        value(SearchCondition::Constant, tag(CONSTANT)),
        tag("()"),
        separator,
    ))(s)
    .map(|(_, (_, m, _, s))| (s.trim(), Ok(m)))
}

fn upper_case(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        value(SearchCondition::UpperCase, tag(UPPER_CASE)),
        tag("()"),
        separator,
    ))(s)
    .map(|(_, (_, m, _, s))| (s.trim(), Ok(m)))
}

fn upper_camel_case(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        value(SearchCondition::UpperCamelCase, tag(UPPER_CAMEL_CASE)),
        tag("()"),
        separator,
    ))(s)
    .map(|(_, (_, m, _, s))| (s.trim(), Ok(m)))
}

fn upper_kebab_case(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        value(SearchCondition::UpperKebabCase, tag(UPPER_KEBAB_CASE)),
        tag("()"),
        separator,
    ))(s)
    .map(|(_, (_, m, _, s))| (s.trim(), Ok(m)))
}

fn upper_snake_case(s: &str) -> IResult<&str, Result<SearchCondition>> {
    tuple((
        space0,
        value(SearchCondition::UpperSnakeCase, tag(UPPER_SNAKE_CASE)),
        tag("()"),
        separator,
    ))(s)
    .map(|(_, (_, m, _, s))| (s.trim(), Ok(m)))
}

#[cfg(test)]
mod tests {
    use crate::parser::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            parse(
                "test | \
                 'ignore_case(test)' | \
                 test2 | \
                 number() | \
                 number() == 10 | \
                 number() == env.PATH_NAME | \
                 number() != 10 | \
                 number() > 10 | \
                 number() >= 10 | \
                 number() < 10 | \
                 number() <= 10 | \
                 line.length() == 10 | \
                 line.length() != 10 | \
                 line.length() > 10 | \
                 line.length() >= 10 | \
                 line.length() < 10 | \
                 line.length() <= 10 | \
                 line.bytelength() == 10 | \
                 line.bytelength() != 10 | \
                 line.bytelength() > 10 | \
                 line.bytelength() >= 10 | \
                 line.bytelength() < 10 | \
                 line.bytelength() <= 10 | \
                 line.contains(test) | \
                 line.regex(test) | \
                 line.starts_with(test) | \
                 line.ends_with(test) | \
                 line.invert_match(test) | \
                 line.invert_match_regex(test) | \
                 contains(test) | \
                 ignore_case(test) | \
                 regex(test) | \
                 whole_word(test) | \
                 starts_with(test) | \
                 ends_with(test) | \
                 invert_match(test) | \
                 invert_match_regex(test) | \
                 replace(from, to) | \
                 camel_case() | \
                 kebab_case() | \
                 snake_case() | \
                 trim_end() | \
                 trim_start() | \
                 trim() | \
                 lower_case() | \
                 constant() | \
                 upper_case() | \
                 upper_camel_case() | \
                 upper_kebab_case() | \
                 upper_snake_case() | \
                 insert(1, $) | \
                 update(value) | \
                 delete(1, 3) | \
                 test"
            )
            .unwrap(),
            vec![
                SearchCondition::Exact("test".to_string()),
                SearchCondition::Exact("ignore_case(test)".to_string()),
                SearchCondition::Exact("test2".to_string()),
                SearchCondition::Number(None),
                SearchCondition::Number(Some(Op::Eq(Value::Num(10)))),
                SearchCondition::Number(Some(Op::Eq(Value::Env("PATH_NAME".to_string())))),
                SearchCondition::Number(Some(Op::Ne(Value::Num(10)))),
                SearchCondition::Number(Some(Op::Gt(Value::Num(10)))),
                SearchCondition::Number(Some(Op::Gte(Value::Num(10)))),
                SearchCondition::Number(Some(Op::Lt(Value::Num(10)))),
                SearchCondition::Number(Some(Op::Lte(Value::Num(10)))),
                SearchCondition::LineLength(Some(Op::Eq(Value::Num(10)))),
                SearchCondition::LineLength(Some(Op::Ne(Value::Num(10)))),
                SearchCondition::LineLength(Some(Op::Gt(Value::Num(10)))),
                SearchCondition::LineLength(Some(Op::Gte(Value::Num(10)))),
                SearchCondition::LineLength(Some(Op::Lt(Value::Num(10)))),
                SearchCondition::LineLength(Some(Op::Lte(Value::Num(10)))),
                SearchCondition::LineByteLength(Some(Op::Eq(Value::Num(10)))),
                SearchCondition::LineByteLength(Some(Op::Ne(Value::Num(10)))),
                SearchCondition::LineByteLength(Some(Op::Gt(Value::Num(10)))),
                SearchCondition::LineByteLength(Some(Op::Gte(Value::Num(10)))),
                SearchCondition::LineByteLength(Some(Op::Lt(Value::Num(10)))),
                SearchCondition::LineByteLength(Some(Op::Lte(Value::Num(10)))),
                SearchCondition::LineContains("test".to_string()),
                SearchCondition::LineRegex("test".to_string()),
                SearchCondition::LineStartsWith("test".to_string()),
                SearchCondition::LineEndsWith("test".to_string()),
                SearchCondition::LineInvertMatch("test".to_string()),
                SearchCondition::LineInvertMatchRegex("test".to_string()),
                SearchCondition::Contains("test".to_string()),
                SearchCondition::IgnoreCase("test".to_string()),
                SearchCondition::Regex("test".to_string()),
                SearchCondition::WholeWord("test".to_string()),
                SearchCondition::StartsWith("test".to_string()),
                SearchCondition::EndsWith("test".to_string()),
                SearchCondition::InvertMatch("test".to_string()),
                SearchCondition::InvertMatchRegex("test".to_string()),
                SearchCondition::Replace("from".to_string(), "to".to_string()),
                SearchCondition::CamelCase,
                SearchCondition::KebabCase,
                SearchCondition::SnakeCase,
                SearchCondition::TrimEnd,
                SearchCondition::TrimStart,
                SearchCondition::Trim,
                SearchCondition::LowerCase,
                SearchCondition::Constant,
                SearchCondition::UpperCase,
                SearchCondition::UpperCamelCase,
                SearchCondition::UpperKebabCase,
                SearchCondition::UpperSnakeCase,
                SearchCondition::Insert(1, "$".to_string()),
                SearchCondition::Update("value".to_string()),
                SearchCondition::Delete(1, 3),
                SearchCondition::Exact("test".to_string()),
            ]
        );
    }
}
