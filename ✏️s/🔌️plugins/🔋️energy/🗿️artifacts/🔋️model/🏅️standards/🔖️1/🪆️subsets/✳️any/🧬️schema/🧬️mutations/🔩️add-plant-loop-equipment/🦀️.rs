//! 🔩️ Energy model mutation — `AddPlantLoopEquipment`: Puts one central-plant equipment id on a plant loop. ⚠️ This is the ONE reference in the whole HVAC group that is NOT checked against a collection: `Model` has no chiller, boiler or pump type at all (vocabulary §5.4), so the id is opaque and only the list shape is enforced. Declared and documented rather than silently validated against nothing; when a plant-equipment collection lands this gains a `g3_chk_reference` and the refusal vector below becomes a referential one.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔩️ `add-plant-loop-equipment` payload. Puts one central-plant equipment id on a plant loop. ⚠️ This is the ONE reference in the whole HVAC group that is NOT checked against a collection: `Model` has no chiller, boiler or pump type at all (vocabulary §5.4), so the id is opaque and only the list shape is enforced. Declared and documented rather than silently validated against nothing; when a plant-equipment collection lands this gains a `g3_chk_reference` and the refusal vector below becomes a referential one.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-plant-loop-equipment")]
pub struct AddPlantLoopEquipment {
    pub id: crate::model::EntityId,
    pub equipment_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_plant_loop_equipment(id: crate::model::EntityId, equipment_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::AddPlantLoopEquipment(AddPlantLoopEquipment { id, equipment_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for AddPlantLoopEquipment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "plant-loop", kind: "add-plant-loop-equipment", record: "AddedPlantLoopEquipment" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Add plant equipment {} to plant loop {}", self.equipment_id.0, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string(), self.equipment_id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
