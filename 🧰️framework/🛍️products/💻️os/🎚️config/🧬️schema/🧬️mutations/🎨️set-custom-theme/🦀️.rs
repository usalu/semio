//! 🎨️ Sets or removes one OS-wide custom theme.
use super::super::super::{UiPreferences, UiPreferencesDiff, UiTheme};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetCustomTheme {
    pub theme_id: String,
    pub theme: Option<UiTheme>,
}
pub fn set_custom_theme(theme_id: impl Into<String>, theme: Option<UiTheme>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetCustomTheme(SetCustomTheme { theme_id: theme_id.into(), theme })
}
keyed_setting_impl!(SetCustomTheme, SetCustomTheme, custom_themes, theme_id, theme, "set-custom-theme", "custom-theme", "custom theme", "custom-themes");
