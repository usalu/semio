//! 🛏️ Energy model mutation — `CreateRoomAirModelAssignment`: Overrides the room air model of one zone. This collection carries no id of its own — `zone_id` IS the key (vocabulary §5.5), so it is a keyed map and a second override for one zone is a duplicate.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛏️ `create-room-air-model-assignment` payload. Overrides the room air model of one zone. This collection carries no id of its own — `zone_id` IS the key (vocabulary §5.5), so it is a keyed map and a second override for one zone is a duplicate.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-room-air-model-assignment")]
pub struct CreateRoomAirModelAssignment {
    pub zone_id: crate::model::EntityId,
    pub model: crate::model::RoomAirModelType,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_room_air_model_assignment<Op: From<CreateRoomAirModelAssignment>>(zone_id: crate::model::EntityId, model: crate::model::RoomAirModelType, index: Option<u32>) -> Op {
    CreateRoomAirModelAssignment { zone_id, model, index }.into()
}
//#endregion 🔖️Mutation
