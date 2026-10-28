//! 🪚️ Energy model mutation — `DeleteSurface`: Removes one surface and CASCADES: every fenestration hosted on it and every adjacency pair naming it go with it, reported at info level as `mutation.cascade`. It still RESTRICTS on the one reference a cascade could not answer for — another surface naming this one as its interzone partner — because silently rewriting that surface's boundary condition is a physics decision no delete may take.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪚️ `delete-surface` payload. Removes one surface and CASCADES: every fenestration hosted on it and every adjacency pair naming it go with it, reported at info level as `mutation.cascade`. It still RESTRICTS on the one reference a cascade could not answer for — another surface naming this one as its interzone partner — because silently rewriting that surface's boundary condition is a physics decision no delete may take.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-surface")]
pub struct DeleteSurface {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_surface<Op: From<DeleteSurface>>(id: crate::model::EntityId) -> Op {
    DeleteSurface { id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
