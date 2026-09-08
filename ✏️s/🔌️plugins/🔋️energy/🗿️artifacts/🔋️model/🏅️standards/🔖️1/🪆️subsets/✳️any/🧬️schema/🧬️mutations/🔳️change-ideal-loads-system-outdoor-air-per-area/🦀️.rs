//! 🔳️ Energy model mutation — `ChangeIdealLoadsSystemOutdoorAirPerArea`: Sets the ventilation rate the system draws per square metre of floor, in m³/s·m².

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔳️ `change-ideal-loads-system-outdoor-air-per-area` payload. Sets the ventilation rate the system draws per square metre of floor, in m³/s·m².
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-outdoor-air-per-area")]
pub struct ChangeIdealLoadsSystemOutdoorAirPerArea {
    pub id: crate::model::EntityId,
    pub new_outdoor_air_per_area_m3_s_m2: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_ideal_loads_system_outdoor_air_per_area(id: crate::model::EntityId, new_outdoor_air_per_area_m3_s_m2: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeIdealLoadsSystemOutdoorAirPerArea(ChangeIdealLoadsSystemOutdoorAirPerArea { id, new_outdoor_air_per_area_m3_s_m2 })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeIdealLoadsSystemOutdoorAirPerArea {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ideal-loads-system", kind: "change-ideal-loads-system-outdoor-air-per-area", record: "ChangedIdealLoadsSystemOutdoorAirPerArea" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change ideal loads system {} outdoor air rate per floor area to {:?}", self.id.0, self.new_outdoor_air_per_area_m3_s_m2)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
