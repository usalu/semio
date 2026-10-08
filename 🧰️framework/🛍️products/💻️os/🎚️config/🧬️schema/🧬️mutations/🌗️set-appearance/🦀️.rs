//! 🌗️ Sets or clears the OS-wide appearance preference.
use super::super::super::{SettingEdit, UiAppearance, UiPreferences, UiPreferencesDiff};
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
impl protocol::MutationKind<UiPreferences, UiPreferencesConfigMutation> for SetAppearance {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "appearance", kind: "set-appearance", record: "Set" };

    fn diff(&self, base: &UiPreferences) -> protocol::MutationOutcome<UiPreferencesDiff> {
        if base.appearance == self.appearance {
            return protocol::MutationOutcome::new(UiPreferencesDiff::default()).warning("mutation.no-op", "appearance is already selected.");
        }
        protocol::MutationOutcome::new(UiPreferencesDiff { appearance: Some(SettingEdit::new(self.appearance.clone())), ..UiPreferencesDiff::default() })
    }

    fn inverse(&self, base: &UiPreferences) -> Result<Vec<UiPreferencesConfigMutation>, semio_framework_value::ValueError> {
        Ok(vec![UiPreferencesConfigMutation::SetAppearance(Self { appearance: base.appearance.clone() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set appearance to {:?}", self.appearance), &format!("Erscheinungsbild auf {:?} setzen", self.appearance))
    }

    fn target(&self) -> Vec<String> {
        vec!["appearance".to_string()]
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️sets-appearance/🦀️.rs"]
mod tests_sets_appearance;

#[cfg(test)]
#[path = "🧪️tests/🟰️keeps-appearance/🦀️.rs"]
mod tests_keeps_appearance;
//#endregion 🧪️Tests
