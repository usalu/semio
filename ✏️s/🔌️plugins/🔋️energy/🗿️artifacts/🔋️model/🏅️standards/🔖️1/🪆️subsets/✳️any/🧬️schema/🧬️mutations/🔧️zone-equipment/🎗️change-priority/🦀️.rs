//! 🎗️ Energy model mutation — `ChangeZoneEquipmentPriority`: Sets the rank this equipment holds in its zone's own equipment list.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎗️ `change-zone-equipment-priority` payload. Sets the rank this equipment holds in its zone's own equipment list.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-equipment-priority")]
pub struct ChangeZoneEquipmentPriority {
    pub id: crate::model::EntityId,
    pub new_priority: u8,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_zone_equipment_priority<Op: From<ChangeZoneEquipmentPriority>>(id: crate::model::EntityId, new_priority: u8) -> Op {
    ChangeZoneEquipmentPriority { id, new_priority }.into()
}
//#endregion 🔖️Mutation
