//! 🖼️ Sets or clears the OS-wide theme selection.
use super::super::super::{UiPreferences, UiPreferencesDiff};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetTheme {
    pub theme_id: Option<String>,
}
pub fn set_theme(theme_id: Option<String>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetTheme(SetTheme { theme_id })
}
optional_setting_impl!(SetTheme, SetTheme, theme_id, "set-theme", "theme", "theme", "theme-id");
