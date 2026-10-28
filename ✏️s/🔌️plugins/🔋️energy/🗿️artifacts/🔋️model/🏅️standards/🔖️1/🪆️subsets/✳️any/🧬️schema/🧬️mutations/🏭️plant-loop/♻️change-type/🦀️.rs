//! ♻️ Energy model mutation — `ChangePlantLoopType`: Swaps which fluid duty the loop carries.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ♻️ `change-plant-loop-type` payload. Swaps which fluid duty the loop carries.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-plant-loop-type")]
pub struct ChangePlantLoopType {
    pub id: crate::model::EntityId,
    pub new_loop_type: crate::model::PlantLoopType,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_plant_loop_type<Op: From<ChangePlantLoopType>>(id: crate::model::EntityId, new_loop_type: crate::model::PlantLoopType) -> Op {
    ChangePlantLoopType { id, new_loop_type }.into()
}
//#endregion 🔖️Mutation
