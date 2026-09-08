//! ⌨️ Sets or removes one OS-wide keybinding override.
use super::super::super::UiPreferencesDiff;
use super::super::UiPreferencesConfigMutation;
use semio_framework_os_shell::UiPreferences;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetKeybindingOverride {
    pub control_id: String,
    pub keys: Option<String>,
}
pub fn set_keybinding_override(control_id: impl Into<String>, keys: Option<String>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetKeybindingOverride(SetKeybindingOverride { control_id: control_id.into(), keys })
}
keyed_setting_impl!(SetKeybindingOverride, SetKeybindingOverride, keybinding_overrides, control_id, keys, "set-keybinding-override", "keybinding-override", "keybinding override", "keybinding-overrides");
