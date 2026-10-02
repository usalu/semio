//! 🌡️ Energy model mutation — `ChangeGroundTemperatureBuildingSurface`: Sets one month's ground temperature under the building, read by the ground heat transfer solve.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌡️ `change-ground-temperature-building-surface` payload. Sets one month's ground temperature under the building, read by the ground heat transfer solve.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ground-temperature-building-surface")]
pub struct ChangeGroundTemperatureBuildingSurface {
    pub month: u8,
    pub new_temperature_c: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_ground_temperature_building_surface(month: u8, new_temperature_c: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeGroundTemperatureBuildingSurface(ChangeGroundTemperatureBuildingSurface { month, new_temperature_c })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeGroundTemperatureBuildingSurface {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ground-temperature", kind: "change-ground-temperature-building-surface", record: "ChangedGroundTemperatureBuildingSurface" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change building-surface ground temperature of month {} to {} °C", self.month, self.new_temperature_c), &format!("Erdreichtemperatur unter dem Gebäude im Monat {} auf {} °C ändern", self.month, self.new_temperature_c))
    }
}
//#endregion 🔖️Mutation
