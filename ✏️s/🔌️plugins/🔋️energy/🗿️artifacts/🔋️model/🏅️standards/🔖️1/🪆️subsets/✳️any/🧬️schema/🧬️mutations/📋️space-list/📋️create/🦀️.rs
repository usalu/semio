//! 📋️ Energy model mutation — `CreateSpaceList`: Adds one named grouping of spaces — the handle a report or an assignment addresses many spaces by at once.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📋️ `create-space-list` payload. Adds one named grouping of spaces — the handle a report or an assignment addresses many spaces by at once.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-space-list")]
pub struct CreateSpaceList {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub name: String,
    pub space_ids: Vec<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_space_list<Op: From<CreateSpaceList>>(index: u32, id: crate::model::EntityId, name: String, space_ids: Vec<crate::model::EntityId>) -> Op {
    CreateSpaceList { index, id, name, space_ids }.into()
}
//#endregion 🔖️Mutation
