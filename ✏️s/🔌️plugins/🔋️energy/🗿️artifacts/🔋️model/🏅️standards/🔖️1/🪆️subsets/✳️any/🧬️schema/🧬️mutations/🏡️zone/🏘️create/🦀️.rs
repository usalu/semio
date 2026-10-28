//! 🏘️ Energy model mutation — `CreateZone`: Adds one thermal zone the caller has already chosen an id for — no mutation mints an id, so an undo can name the same zone again.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏘️ `create-zone` payload. Adds one thermal zone the caller has already chosen an id for — no mutation mints an id, so an undo can name the same zone again.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-zone")]
pub struct CreateZone {
    pub id: crate::model::EntityId,
    pub name: String,
    pub volume_m3: f64,
    pub multiplier: u32,
    pub conditioned: bool,
    pub part_of_total_floor_area: bool,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_zone<Op: From<CreateZone>>(id: crate::model::EntityId, name: String, volume_m3: f64, multiplier: u32, conditioned: bool, part_of_total_floor_area: bool, index: Option<u32>) -> Op {
    CreateZone { id, name, volume_m3, multiplier, conditioned, part_of_total_floor_area, index }.into()
}
//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod original_creation_tests;
