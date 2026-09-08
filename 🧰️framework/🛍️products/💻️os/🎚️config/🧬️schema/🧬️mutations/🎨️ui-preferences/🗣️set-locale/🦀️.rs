//! 🗣️ Sets or clears the OS-wide locale preference.
use super::super::super::UiPreferencesDiff;
use super::super::UiPreferencesConfigMutation;
use semio_framework_os_shell::{UiLocale, UiPreferences};
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetLocale {
    pub locale: Option<UiLocale>,
}
pub fn set_locale(locale: Option<UiLocale>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetLocale(SetLocale { locale })
}
optional_setting_impl!(SetLocale, SetLocale, locale, "set-locale", "locale", "locale", "locale");
