use std::fmt::{Debug, Display};
use std::ops::Range;

mod exact;
mod ignore_case;
mod number;
mod regex;
mod whole_word;

pub use self::exact::Exact;
pub use self::ignore_case::IgnoreCase;
pub use self::number::Number;
pub use self::regex::Regex;
pub use self::whole_word::WholeWord;

pub trait Matcher: Debug + Display + Send + Sync + 'static {
    fn find(&self, text: &str) -> Vec<(String, Range<usize>)>;
}

impl PartialEq for dyn Matcher {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
