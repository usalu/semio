//! 📖️ Sets or clears the OS-wide terminology preference.
use super::super::super::{SettingEdit, UiPreferences, UiPreferencesDiff};
use super::super::UiPreferencesConfigMutation;
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
impl protocol::MutationKind<UiPreferences, UiPreferencesConfigMutation> for SetTerminology {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "terminology", kind: "set-terminology", record: "Set" };

    fn diff(&self, base: &UiPreferences) -> protocol::MutationOutcome<UiPreferencesDiff> {
        if base.terminology == self.terminology {
            return protocol::MutationOutcome::new(UiPreferencesDiff::default()).warning("mutation.no-op", "terminology is already selected.");
        }
        protocol::MutationOutcome::new(UiPreferencesDiff { terminology: Some(SettingEdit::new(self.terminology.clone())), ..UiPreferencesDiff::default() })
    }

    fn inverse(&self, base: &UiPreferences) -> Result<Vec<UiPreferencesConfigMutation>, semio_framework_value::ValueError> {
        Ok(vec![UiPreferencesConfigMutation::Terminology(Self { terminology: base.terminology.clone() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set terminology to {:?}", self.terminology), &format!("Terminologie auf {:?} setzen", self.terminology))
    }

    fn target(&self) -> Vec<String> {
        vec!["terminology".to_string()]
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️sets-terminology/🦀️.rs"]
mod tests_sets_terminology;

#[cfg(test)]
#[path = "🧪️tests/🟰️keeps-terminology/🦀️.rs"]
mod tests_keeps_terminology;
//#endregion 🧪️Tests
