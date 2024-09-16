use super::{nested_setting::NestedSetting, whitespace::Whitespace};
use std::{fmt::Display, ops::Deref};

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct NestedSettings {
    pub(crate) prev_line_whitespace: Whitespace,
    pub(crate) settings: Vec<NestedSetting>,
}

impl Deref for NestedSettings {
    type Target = Vec<NestedSetting>;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl Display for NestedSettings {
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
