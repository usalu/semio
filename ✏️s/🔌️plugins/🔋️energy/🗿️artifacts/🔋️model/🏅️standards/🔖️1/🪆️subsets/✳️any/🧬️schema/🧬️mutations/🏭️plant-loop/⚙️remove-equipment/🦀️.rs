//! ⚙️ Energy model mutation — `RemovePlantLoopEquipment`: Takes one central-plant equipment id off a plant loop — the repair a document that arrived with a dangling `equipment_ids` entry needs, since no collection backs those ids (vocabulary §5.4). Refused when the loop does not list the id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚙️ `remove-plant-loop-equipment` payload. Takes one central-plant equipment id off a plant loop — the repair a document that arrived with a dangling `equipment_ids` entry needs, since no collection backs those ids (vocabulary §5.4). Refused when the loop does not list the id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-plant-loop-equipment")]
pub struct RemovePlantLoopEquipment {
    pub id: crate::model::EntityId,
    pub equipment_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_plant_loop_equipment<Op: From<RemovePlantLoopEquipment>>(id: crate::model::EntityId, equipment_id: crate::model::EntityId) -> Op {
    RemovePlantLoopEquipment { id, equipment_id }.into()
}

//#endregion 🔖️Mutation
