//! 🍂️ Energy model mutation — `DeleteOutdoorAirSystem`: Drops one outdoor air system off its loop. Refused when no system carries the id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🍂️ `delete-outdoor-air-system` payload. Drops one outdoor air system off its loop. Refused when no system carries the id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-outdoor-air-system")]
pub struct DeleteOutdoorAirSystem {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_outdoor_air_system<Op: From<DeleteOutdoorAirSystem>>(id: crate::model::EntityId) -> Op {
    DeleteOutdoorAirSystem { id }.into()
}

//#endregion 🔖️Mutation
