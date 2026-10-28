//! 🍧️ Energy model mutation — `ChangeZoneEquipmentCoolingCapacity`: Sets the cooling output this equipment can deliver, in watts.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🍧️ `change-zone-equipment-cooling-capacity` payload. Sets the cooling output this equipment can deliver, in watts.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-equipment-cooling-capacity")]
pub struct ChangeZoneEquipmentCoolingCapacity {
    pub id: crate::model::EntityId,
    pub new_cooling_capacity_w: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_zone_equipment_cooling_capacity<Op: From<ChangeZoneEquipmentCoolingCapacity>>(id: crate::model::EntityId, new_cooling_capacity_w: f64) -> Op {
    ChangeZoneEquipmentCoolingCapacity { id, new_cooling_capacity_w }.into()
}
//#endregion 🔖️Mutation
