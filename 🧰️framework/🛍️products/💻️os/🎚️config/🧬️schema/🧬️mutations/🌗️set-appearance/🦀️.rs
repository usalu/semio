//! 🌗️ Sets or clears the OS-wide appearance preference.
use super::super::super::{UiAppearance, UiPreferences, UiPreferencesDiff};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetAppearance {
    pub appearance: Option<UiAppearance>,
}
pub fn set_appearance(appearance: Option<UiAppearance>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetAppearance(SetAppearance { appearance })
}
optional_setting_impl!(SetAppearance, SetAppearance, appearance, "set-appearance", "appearance", "appearance", "appearance");
