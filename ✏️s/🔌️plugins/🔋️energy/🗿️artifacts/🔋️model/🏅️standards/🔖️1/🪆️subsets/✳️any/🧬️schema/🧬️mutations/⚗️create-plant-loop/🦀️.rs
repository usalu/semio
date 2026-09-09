//! ⚗️ Energy model mutation — `CreatePlantLoop`: Creates one plant loop. ⚠️ `equipment_ids` is the ONE reference this group cannot check: `Model` carries no chiller/boiler/pump collection at all (vocabulary §5.4), so the ids are opaque here — the list is only held to being ascending, free of duplicates and free of the unset id zero. When a plant-equipment collection lands, add a `g3_chk_reference` over it; the hole is recorded in this ticket's `📓️w7-g3-hvac.md` rather than papered over.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚗️ `create-plant-loop` payload. Creates one plant loop. ⚠️ `equipment_ids` is the ONE reference this group cannot check: `Model` carries no chiller/boiler/pump collection at all (vocabulary §5.4), so the ids are opaque here — the list is only held to being ascending, free of duplicates and free of the unset id zero. When a plant-equipment collection lands, add a `g3_chk_reference` over it; the hole is recorded in this ticket's `📓️w7-g3-hvac.md` rather than papered over.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-plant-loop")]
pub struct CreatePlantLoop {
    pub id: crate::model::EntityId,
    pub name: String,
    pub loop_type: crate::model::PlantLoopType,
    pub supply_temperature_c: f64,
    pub return_temperature_c: f64,
    pub design_flow_kg_s: f64,
    pub equipment_ids: Vec<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_plant_loop(
    id: crate::model::EntityId,
    name: String,
    loop_type: crate::model::PlantLoopType,
    supply_temperature_c: f64,
    return_temperature_c: f64,
    design_flow_kg_s: f64,
    equipment_ids: Vec<crate::model::EntityId>,
) -> EnergyModelMutation {
    EnergyModelMutation::CreatePlantLoop(CreatePlantLoop { id, name, loop_type, supply_temperature_c, return_temperature_c, design_flow_kg_s, equipment_ids })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreatePlantLoop {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "plant-loop", kind: "create-plant-loop", record: "CreatedPlantLoop" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create plant loop {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
