//! 🧃️ Energy model mutation — `ChangeSolarThermalSystemStorageVolume`: Sets buffer volume (m³) on one solar thermal system, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧃️ `change-solar-thermal-system-storage-volume` payload. Sets buffer volume (m³) on one solar thermal system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-solar-thermal-system-storage-volume")]
pub struct ChangeSolarThermalSystemStorageVolume {
    pub id: crate::model::EntityId,
    pub new_storage_volume_m3: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_solar_thermal_system_storage_volume<Op: From<ChangeSolarThermalSystemStorageVolume>>(id: crate::model::EntityId, new_storage_volume_m3: f64) -> Op {
    ChangeSolarThermalSystemStorageVolume { id, new_storage_volume_m3 }.into()
}
//#endregion 🔖️Mutation
