//! 🛠️ Energy model mutation — `CreateZoneEquipment`: Adds one piece of zone equipment to a zone's own equipment list. `priority` is its rank in that list, so it starts at one.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛠️ `create-zone-equipment` payload. Adds one piece of zone equipment to a zone's own equipment list. `priority` is its rank in that list, so it starts at one.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-zone-equipment")]
pub struct CreateZoneEquipment {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub equipment_type: crate::model::ZoneEquipmentType,
    pub priority: u8,
    pub heating_capacity_w: f64,
    pub cooling_capacity_w: f64,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_zone_equipment<Op: From<CreateZoneEquipment>>(id: crate::model::EntityId, zone_id: crate::model::EntityId, equipment_type: crate::model::ZoneEquipmentType, priority: u8, heating_capacity_w: f64, cooling_capacity_w: f64, index: Option<u32>) -> Op {
    CreateZoneEquipment { id, zone_id, equipment_type, priority, heating_capacity_w, cooling_capacity_w, index }.into()
}
//#endregion 🔖️Mutation
