//! 🟩️ Energy model mutation — `ChangeSolarThermalSystemCollectorArea`: Sets collector area (m²) on one solar thermal system, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟩️ `change-solar-thermal-system-collector-area` payload. Sets collector area (m²) on one solar thermal system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-solar-thermal-system-collector-area")]
pub struct ChangeSolarThermalSystemCollectorArea {
    pub id: crate::model::EntityId,
    pub new_collector_area_m2: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_solar_thermal_system_collector_area(id: crate::model::EntityId, new_collector_area_m2: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSolarThermalSystemCollectorArea(ChangeSolarThermalSystemCollectorArea { id, new_collector_area_m2 })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSolarThermalSystemCollectorArea {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "solar-thermal-system", kind: "change-solar-thermal-system-collector-area", record: "ChangedSolarThermalSystemCollectorArea" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Solar Thermal System Collector Area of solar thermal system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
