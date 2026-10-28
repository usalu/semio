//! 🪵️ Energy model mutation — `DeleteShadingSurface`: Removes one free-standing solar obstruction. Nothing in the document references a shading surface, so it neither cascades nor restricts.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪵️ `delete-shading-surface` payload. Removes one free-standing solar obstruction. Nothing in the document references a shading surface, so it neither cascades nor restricts.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-shading-surface")]
pub struct DeleteShadingSurface {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_shading_surface<Op: From<DeleteShadingSurface>>(id: crate::model::EntityId) -> Op {
    DeleteShadingSurface { id }.into()
}

//#endregion 🔖️Mutation
