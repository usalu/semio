//! 🥀️ Energy model mutation — `DeleteAirLoop`: Drops one air loop. Refused while an outdoor air system still names it, rather than cascading — the outdoor air system is a document of its own and its owner decides its fate.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🥀️ `delete-air-loop` payload. Drops one air loop. Refused while an outdoor air system still names it, rather than cascading — the outdoor air system is a document of its own and its owner decides its fate.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-air-loop")]
pub struct DeleteAirLoop {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_air_loop<Op: From<DeleteAirLoop>>(id: crate::model::EntityId) -> Op {
    DeleteAirLoop { id }.into()
}

//#endregion 🔖️Mutation
