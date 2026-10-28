//! 🧺️ Energy model mutation — `DeleteRoomAirModelAssignment`: Drops one zone's room air model override, so the zone falls back to the engine's own well-mixed default.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧺️ `delete-room-air-model-assignment` payload. Drops one zone's room air model override, so the zone falls back to the engine's own well-mixed default.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-room-air-model-assignment")]
pub struct DeleteRoomAirModelAssignment {
    pub zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_room_air_model_assignment<Op: From<DeleteRoomAirModelAssignment>>(zone_id: crate::model::EntityId) -> Op {
    DeleteRoomAirModelAssignment { zone_id }.into()
}

//#endregion 🔖️Mutation
