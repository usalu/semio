//! 🔧️ Energy model mutation — `ChangeZoneEquipmentType`: Swaps the catalog type the assignment names. The eight-way enum is one scalar, not eight fields, so it changes as a whole.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔧️ `change-zone-equipment-type` payload. Swaps the catalog type the assignment names. The eight-way enum is one scalar, not eight fields, so it changes as a whole.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-equipment-type")]
pub struct ChangeZoneEquipmentType {
    pub id: crate::model::EntityId,
    pub new_equipment_type: crate::model::ZoneEquipmentType,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_zone_equipment_type<Op: From<ChangeZoneEquipmentType>>(id: crate::model::EntityId, new_equipment_type: crate::model::ZoneEquipmentType) -> Op {
    ChangeZoneEquipmentType { id, new_equipment_type }.into()
}
//#endregion 🔖️Mutation
