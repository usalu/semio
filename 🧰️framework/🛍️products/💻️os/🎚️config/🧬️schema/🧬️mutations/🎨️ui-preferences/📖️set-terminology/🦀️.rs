//! 📖️ Sets or clears the OS-wide terminology preference.
use super::super::super::UiPreferencesDiff;
use super::super::UiPreferencesConfigMutation;
use semio_framework_os_shell::UiPreferences;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetTerminology {
    pub terminology: Option<String>,
}
pub fn set_terminology(terminology: Option<String>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetTerminology(SetTerminology { terminology })
}
optional_setting_impl!(SetTerminology, SetTerminology, terminology, "set-terminology", "terminology", "terminology", "terminology");
