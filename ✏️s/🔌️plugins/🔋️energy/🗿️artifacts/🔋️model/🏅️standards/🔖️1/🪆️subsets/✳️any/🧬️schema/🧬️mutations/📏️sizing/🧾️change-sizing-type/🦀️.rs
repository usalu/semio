//! 🧾️ Energy model mutation — `ChangeSizingObjectSizingType`: Swaps which load the sizing run solves for.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧾️ `change-sizing-object-sizing-type` payload. Swaps which load the sizing run solves for.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-sizing-object-sizing-type")]
pub struct ChangeSizingObjectSizingType {
    pub id: crate::model::EntityId,
    pub new_sizing_type: crate::model::SizingType,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_sizing_object_sizing_type<Op: From<ChangeSizingObjectSizingType>>(id: crate::model::EntityId, new_sizing_type: crate::model::SizingType) -> Op {
    ChangeSizingObjectSizingType { id, new_sizing_type }.into()
}
//#endregion 🔖️Mutation
