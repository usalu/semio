//! 🪑️ Energy model mutation — `CreateSpace`: Adds one space inside an existing zone. The owning zone must already exist, so the document never carries a space whose zone the reports cannot resolve.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪑️ `create-space` payload. Adds one space inside an existing zone. The owning zone must already exist, so the document never carries a space whose zone the reports cannot resolve.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-space")]
pub struct CreateSpace {
    pub id: crate::model::EntityId,
    pub name: String,
    pub zone_id: crate::model::EntityId,
    pub floor_area_m2: f64,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_space<Op: From<CreateSpace>>(id: crate::model::EntityId, name: String, zone_id: crate::model::EntityId, floor_area_m2: f64, index: Option<u32>) -> Op {
    CreateSpace { id, name, zone_id, floor_area_m2, index }.into()
}
//#endregion 🔖️Mutation
