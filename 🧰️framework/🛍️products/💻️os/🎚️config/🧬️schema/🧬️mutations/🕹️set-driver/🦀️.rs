//! 🕹️ Sets or clears the OS-wide driver selection.
use super::super::super::{SettingEdit, UiPreferences, UiPreferencesDiff};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetDriver {
    pub driver_id: Option<String>,
}
pub fn set_driver(driver_id: Option<String>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetDriver(SetDriver { driver_id })
}
impl protocol::MutationKind<UiPreferences, UiPreferencesConfigMutation> for SetDriver {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "driver", kind: "set-driver", record: "Set" };

    fn diff(&self, base: &UiPreferences) -> protocol::MutationOutcome<UiPreferencesDiff> {
        if base.driver_id == self.driver_id {
            return protocol::MutationOutcome::new(UiPreferencesDiff::default()).warning("mutation.no-op", "driver is already selected.");
        }
        protocol::MutationOutcome::new(UiPreferencesDiff { driver_id: Some(SettingEdit::new(self.driver_id.clone())), ..UiPreferencesDiff::default() })
    }

    fn inverse(&self, base: &UiPreferences) -> Result<Vec<UiPreferencesConfigMutation>, semio_framework_value::ValueError> {
        Ok(vec![UiPreferencesConfigMutation::SetDriver(Self { driver_id: base.driver_id.clone() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set driver to {:?}", self.driver_id), &format!("Treiber auf {:?} setzen", self.driver_id))
    }

    fn target(&self) -> Vec<String> {
        vec!["driver-id".to_string()]
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️sets-driver/🦀️.rs"]
mod tests_sets_driver;

#[cfg(test)]
#[path = "🧪️tests/🟰️keeps-driver/🦀️.rs"]
mod tests_keeps_driver;
//#endregion 🧪️Tests
