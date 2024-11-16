mod contains;
mod ends_with;
mod invert_match;
mod invert_match_regex;
mod length;
mod regex;
mod starts_with;

pub use self::contains::Contains;
pub use self::ends_with::EndsWith;
pub use self::invert_match::InvertMatch;
pub use self::invert_match_regex::InvertMatchRegex;
pub use self::length::Length;
pub use self::regex::Regex;
pub use self::starts_with::StartsWith;
use std::fmt::{Debug, Display};

pub trait Filter: Debug + Display + Send + Sync + 'static {
    fn filter(&self, text: &str) -> bool;
}

impl PartialEq for dyn Filter {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
