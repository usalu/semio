//! ⚙️ Energy model mutation — `RemovePlantLoopEquipment`: Takes one central-plant equipment id off a plant loop — the repair a document that arrived with a dangling `equipment_ids` entry needs, since no collection backs those ids (vocabulary §5.4). Refused when the loop does not list the id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚙️ `remove-plant-loop-equipment` payload. Takes one central-plant equipment id off a plant loop — the repair a document that arrived with a dangling `equipment_ids` entry needs, since no collection backs those ids (vocabulary §5.4). Refused when the loop does not list the id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-plant-loop-equipment")]
pub struct RemovePlantLoopEquipment {
    pub id: crate::model::EntityId,
    pub equipment_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_plant_loop_equipment(id: crate::model::EntityId, equipment_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::RemovePlantLoopEquipment(RemovePlantLoopEquipment { id, equipment_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RemovePlantLoopEquipment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "plant-loop", kind: "remove-plant-loop-equipment", record: "RemovedPlantLoopEquipment" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Remove plant equipment {} from plant loop {}", self.equipment_id.0, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string(), self.equipment_id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
