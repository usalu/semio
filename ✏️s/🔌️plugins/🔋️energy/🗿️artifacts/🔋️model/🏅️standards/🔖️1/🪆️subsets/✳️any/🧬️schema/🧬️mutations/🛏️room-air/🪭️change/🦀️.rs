//! 🪭️ Energy model mutation — `ChangeRoomAirModel`: Swaps the room air model one zone's override names.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪭️ `change-room-air-model` payload. Swaps the room air model one zone's override names.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-room-air-model")]
pub struct ChangeRoomAirModel {
    pub zone_id: crate::model::EntityId,
    pub new_model: crate::model::RoomAirModelType,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_room_air_model<Op: From<ChangeRoomAirModel>>(zone_id: crate::model::EntityId, new_model: crate::model::RoomAirModelType) -> Op {
    ChangeRoomAirModel { zone_id, new_model }.into()
}
//#endregion 🔖️Mutation
