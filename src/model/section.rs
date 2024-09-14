use super::{entry::Entry, header::Header};
use crate::lexer::{Parsable, ParserOutput};
use nom::multi::many0;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Section<'a> {
    header: Header<'a>,
    entries: Vec<Entry<'a>>,
}

impl<'a> Display for Section<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.header,
            self.entries
                .iter()
                .map(Entry::to_string)
                .collect::<String>()
        )
    }
}

impl<'a> Parsable<'a> for Section<'a> {
    type Output = Self;

    fn parse(input: &'a str) -> ParserOutput<'a, Self::Output> {
        let (next, header) = Header::parse(input)?;
        let (next, entries) = many0(Entry::parse)(next)?;
        let section = Self { header, entries };

        Ok((next, section))
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Parsable;

    use super::Section;

    const SAMPLE_SECTION: &str = r#"[default] # This is my comment
region=us-west-2
output=json"#;

    #[test]
    fn parses_section_with_two_entries_not_nested() {
        let (rest, section) = Section::parse(SAMPLE_SECTION).expect("Should be valid");

        assert!(rest.is_empty());

        let entries = section.entries;

        let first = &entries[0];

        let _ = match first {
            crate::model::entry::Entry::Single(first) => first,
            crate::model::entry::Entry::WithSubsettings(_) => panic!("Should not be a subsection"),
        };

        let second = &entries[1];
        let _ = match second {
            crate::model::entry::Entry::Single(second) => second,
            crate::model::entry::Entry::WithSubsettings(_) => panic!("Should not be a subsection"),
        };
    }
}
