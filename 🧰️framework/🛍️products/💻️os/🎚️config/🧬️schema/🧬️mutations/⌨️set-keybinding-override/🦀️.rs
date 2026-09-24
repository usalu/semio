//! ⌨️ Sets or removes one OS-wide keybinding override.
use super::super::super::{UiPreferences, UiPreferencesDiff};
use super::super::UiPreferencesConfigMutation;
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

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️sets-keybinding/🦀️.rs"]
mod tests_sets_keybinding;

#[cfg(test)]
#[path = "🧪️tests/🟰️keeps-keybinding/🦀️.rs"]
mod tests_keeps_keybinding;
//#endregion 🧪️Tests
