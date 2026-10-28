//! 🪒️ Energy model mutation — `DeleteSizingObject`: Drops one zone's sizing declaration. Refused when no sizing object carries the id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪒️ `delete-sizing-object` payload. Drops one zone's sizing declaration. Refused when no sizing object carries the id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-sizing-object")]
pub struct DeleteSizingObject {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_sizing_object<Op: From<DeleteSizingObject>>(id: crate::model::EntityId) -> Op {
    DeleteSizingObject { id }.into()
}

//#endregion 🔖️Mutation
