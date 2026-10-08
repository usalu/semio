//! 🚗️ Sets or removes one OS-wide custom driver.
use super::super::super::{KeyedEdit, UiDriver, UiPreferences, UiPreferencesDiff};
use super::super::UiPreferencesConfigMutation;
use semio_framework_value_derive::{FromValue, ToValue};
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetCustomDriver {
    pub driver_id: String,
    pub driver: Option<UiDriver>,
}
pub fn set_custom_driver(driver_id: impl Into<String>, driver: Option<UiDriver>) -> UiPreferencesConfigMutation {
    UiPreferencesConfigMutation::SetCustomDriver(SetCustomDriver { driver_id: driver_id.into(), driver })
}
impl protocol::MutationKind<UiPreferences, UiPreferencesConfigMutation> for SetCustomDriver {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "custom-driver", kind: "set-custom-driver", record: "Set" };

    fn diff(&self, base: &UiPreferences) -> protocol::MutationOutcome<UiPreferencesDiff> {
        if base.custom_drivers.get(&self.driver_id) == self.driver.as_ref() {
            return protocol::MutationOutcome::new(UiPreferencesDiff::default()).warning("mutation.no-op", "custom driver already has the requested value.");
        }
        protocol::MutationOutcome::new(UiPreferencesDiff { custom_drivers: vec![KeyedEdit::new(self.driver_id.clone(), self.driver.clone())], ..UiPreferencesDiff::default() })
    }

    fn inverse(&self, base: &UiPreferences) -> Result<Vec<UiPreferencesConfigMutation>, semio_framework_value::ValueError> {
        Ok(vec![UiPreferencesConfigMutation::SetCustomDriver(Self { driver_id: self.driver_id.clone(), driver: base.custom_drivers.get(&self.driver_id).cloned() })])
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set custom driver {:?}", self.driver_id), &format!("Eigener Treiber {:?} setzen", self.driver_id))
    }

    fn target(&self) -> Vec<String> {
        vec!["custom-drivers".to_string(), self.driver_id.clone()]
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/✏️sets-custom-driver/🦀️.rs"]
mod tests_sets_custom_driver;

#[cfg(test)]
#[path = "🧪️tests/🟰️keeps-custom-driver/🦀️.rs"]
mod tests_keeps_custom_driver;
//#endregion 🧪️Tests
