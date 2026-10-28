//! 📛️ Energy model mutation — `RenamePlantLoop`: Sets one plant loop's identity field.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📛️ `rename-plant-loop` payload. Sets one plant loop's identity field.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-plant-loop")]
pub struct RenamePlantLoop {
    pub id: crate::model::EntityId,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn rename_plant_loop<Op: From<RenamePlantLoop>>(id: crate::model::EntityId, new_name: String) -> Op {
    RenamePlantLoop { id, new_name }.into()
}
//#endregion 🔖️Mutation
