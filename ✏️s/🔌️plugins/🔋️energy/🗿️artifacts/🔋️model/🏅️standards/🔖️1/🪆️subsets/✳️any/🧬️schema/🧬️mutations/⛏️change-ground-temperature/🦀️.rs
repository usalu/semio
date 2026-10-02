//! ⛏️ Energy model mutation — `ChangeGroundTemperatureDeep`: Sets the deep ground temperature, read by the ground heat transfer solve.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛏️ `change-ground-temperature-deep` payload. Sets the deep ground temperature, read by the ground heat transfer solve.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ground-temperature-deep")]
pub struct ChangeGroundTemperatureDeep {
    pub new_temperature_c: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_ground_temperature_deep(new_temperature_c: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeGroundTemperatureDeep(ChangeGroundTemperatureDeep { new_temperature_c })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeGroundTemperatureDeep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ground-temperature", kind: "change-ground-temperature-deep", record: "ChangedGroundTemperatureDeep" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change deep ground temperature to {} °C", self.new_temperature_c), &format!("Erdreichtemperatur in der Tiefe auf {} °C ändern", self.new_temperature_c))
    }
}
//#endregion 🔖️Mutation
