//! ⚡️ Energy model mutation — `ChangeEquipmentGainWattsPerArea`: Sets equipment power density (W/m²) on one equipment gain, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚡️ `change-equipment-gain-watts-per-area` payload. Sets equipment power density (W/m²) on one equipment gain, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-equipment-gain-watts-per-area")]
pub struct ChangeEquipmentGainWattsPerArea {
    pub id: crate::model::EntityId,
    pub new_watts_per_area: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_equipment_gain_watts_per_area(id: crate::model::EntityId, new_watts_per_area: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeEquipmentGainWattsPerArea(ChangeEquipmentGainWattsPerArea { id, new_watts_per_area })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeEquipmentGainWattsPerArea {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "equipment-gain", kind: "change-equipment-gain-watts-per-area", record: "ChangedEquipmentGainWattsPerArea" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Equipment Gain Watts Per Area of equipment gain {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
