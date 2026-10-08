//! ⌨️ Sets or removes one OS-wide keybinding override.
use super::super::super::{KeyedEdit, UiPreferences, UiPreferencesDiff};
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
impl protocol::MutationKind<UiPreferences, UiPreferencesConfigMutation> for SetKeybindingOverride {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "keybinding-override", kind: "set-keybinding-override", record: "Set" };

    fn diff(&self, base: &UiPreferences) -> protocol::MutationOutcome<UiPreferencesDiff> {
        if base.keybinding_overrides.get(&self.control_id) == self.keys.as_ref() {
            return protocol::MutationOutcome::new(UiPreferencesDiff::default()).warning("mutation.no-op", "keybinding override already has the requested value.");
        }
        protocol::MutationOutcome::new(UiPreferencesDiff { keybinding_overrides: vec![KeyedEdit::new(self.control_id.clone(), self.keys.clone())], ..UiPreferencesDiff::default() })
    }

    fn inverse(&self, base: &UiPreferences) -> Result<Vec<UiPreferencesConfigMutation>, semio_framework_value::ValueError> {
        Ok(vec![UiPreferencesConfigMutation::SetKeybindingOverride(Self { control_id: self.control_id.clone(), keys: base.keybinding_overrides.get(&self.control_id).cloned() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set keybinding override {:?}", self.control_id), &format!("Tastenkürzel {:?} setzen", self.control_id))
    }

    fn target(&self) -> Vec<String> {
        vec!["keybinding-overrides".to_string(), self.control_id.clone()]
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️sets-keybinding/🦀️.rs"]
mod tests_sets_keybinding;

#[cfg(test)]
#[path = "🧪️tests/🟰️keeps-keybinding/🦀️.rs"]
mod tests_keeps_keybinding;
//#endregion 🧪️Tests
