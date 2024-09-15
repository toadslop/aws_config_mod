use super::{nested_setting::NestedSetting, whitespace::Whitespace};
use std::{fmt::Display, ops::Deref};

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct NestedSettings<'a> {
    pub(crate) prev_line_whitespace: Whitespace<'a>,
    pub(crate) settings: Vec<NestedSetting<'a>>,
}

impl<'a> Deref for NestedSettings<'a> {
    type Target = Vec<NestedSetting<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl<'a> Display for NestedSettings<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.prev_line_whitespace,
            self.settings
                .iter()
                .map(NestedSetting::to_string)
                .collect::<String>()
        )
    }
}
