//! ⛵️ Energy model mutation — `ChangeSolarThermalSystemAzimuth`: Sets azimuth on one solar thermal system, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛵️ `change-solar-thermal-system-azimuth` payload. Sets azimuth on one solar thermal system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-solar-thermal-system-azimuth")]
pub struct ChangeSolarThermalSystemAzimuth {
    pub id: crate::model::EntityId,
    pub new_azimuth_deg: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_solar_thermal_system_azimuth<Op: From<ChangeSolarThermalSystemAzimuth>>(id: crate::model::EntityId, new_azimuth_deg: f64) -> Op {
    ChangeSolarThermalSystemAzimuth { id, new_azimuth_deg }.into()
}
//#endregion 🔖️Mutation
