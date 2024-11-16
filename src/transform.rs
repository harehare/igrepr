use std::fmt::{Debug, Display};
use std::ops::Range;

mod camel_case;
mod delete;
mod insert;
mod kebab_case;
mod lower_case;
mod replace;
mod snake_case;
mod trim;
mod trim_end;
mod trim_start;
mod update;
mod upper_camel_case;
mod upper_case;
mod upper_kebab_case;
mod upper_snake_case;

pub use self::camel_case::CamelCase;
pub use self::delete::Delete;
pub use self::insert::Insert;
pub use self::kebab_case::KebabCase;
pub use self::lower_case::LowerCase;
pub use self::replace::Replace;
pub use self::snake_case::SnakeCase;
pub use self::trim::Trim;
pub use self::trim_end::TrimEnd;
pub use self::trim_start::TrimStart;
pub use self::update::Update;
pub use self::upper_camel_case::UpperCamelCase;
pub use self::upper_case::UpperCase;
pub use self::upper_kebab_case::UpperKebabCase;
pub use self::upper_snake_case::UpperSnakeCase;

pub trait Transform: Debug + Display + Send + Sync + 'static {
    fn transform(&self, text: &str) -> Vec<(String, Range<usize>)>;
}

impl PartialEq for dyn Transform {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
