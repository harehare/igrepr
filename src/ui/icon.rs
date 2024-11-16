pub trait Icon<'a> {
    fn ignore_case(&self) -> &'a str;
    fn regex(&self) -> &'a str;
    fn whole_word(&self) -> &'a str;
    fn replace(&self) -> &'a str;
    fn error(&self) -> &'a str;
    fn filter(&self) -> &'a str;
    fn line(&self) -> &'a str;
    fn search(&self) -> &'a str;
    fn insert(&self) -> &'a str;
    fn delete(&self) -> &'a str;
    fn number(&self) -> &'a str;
}

pub struct CharIcon;

impl<'a> Icon<'a> for CharIcon {
    fn ignore_case(&self) -> &'a str {
        ""
    }

    fn regex(&self) -> &'a str {
        ""
    }

    fn whole_word(&self) -> &'a str {
        ""
    }

    fn replace(&self) -> &'a str {
        ""
    }

    fn error(&self) -> &'a str {
        ""
    }

    fn filter(&self) -> &'a str {
        ""
    }

    fn line(&self) -> &'a str {
        ""
    }
    fn search(&self) -> &'a str {
        ""
    }
    fn insert(&self) -> &'a str {
        ""
    }
    fn delete(&self) -> &'a str {
        ""
    }
    fn number(&self) -> &'a str {
        ""
    }
}

pub struct FontIcon;

impl<'a> Icon<'a> for FontIcon {
    fn ignore_case(&self) -> &'a str {
        "\u{eab1} "
    }

    fn regex(&self) -> &'a str {
        "\u{eb38} "
    }

    fn whole_word(&self) -> &'a str {
        "\u{eb7e} "
    }

    fn replace(&self) -> &'a str {
        "\u{eb3c} "
    }

    fn error(&self) -> &'a str {
        "\u{e654}"
    }

    fn filter(&self) -> &'a str {
        "\u{eaf1}"
    }

    fn line(&self) -> &'a str {
        "\u{ef4c}"
    }
    fn search(&self) -> &'a str {
        "\u{e68f}"
    }
    fn insert(&self) -> &'a str {
        "\u{ec11}"
    }
    fn delete(&self) -> &'a str {
        "\u{ee23}"
    }
    fn number(&self) -> &'a str {
        "\u{f4f7}"
    }
}
