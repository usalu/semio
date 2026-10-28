//! 🏠️ Energy model mutation — `RenameZone`: Sets one zone's identity field. Zone names are the key every EnergyPlus-class report keys on, so a duplicate is refused.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏠️ `rename-zone` payload. Sets one zone's identity field. Zone names are the key every EnergyPlus-class report keys on, so a duplicate is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-zone")]
pub struct RenameZone {
    pub id: crate::model::EntityId,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn rename_zone<Op: From<RenameZone>>(id: crate::model::EntityId, new_name: String) -> Op {
    RenameZone { id, new_name }.into()
}
//#endregion 🔖️Mutation
