//! ✨️ Energy model mutation — `CreatePvSystem`: Adds one photovoltaic array. Capacity, aperture area and the two efficiencies fix the DC-to-AC chain; tilt and azimuth place the plane the incident-solar model integrates over.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ✨️ `create-pv-system` payload. Adds one photovoltaic array. Capacity, aperture area and the two efficiencies fix the DC-to-AC chain; tilt and azimuth place the plane the incident-solar model integrates over.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-pv-system")]
pub struct CreatePvSystem {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub dc_capacity_w: f64,
    pub area_m2: f64,
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
    pub module_efficiency: f64,
    pub inverter_efficiency: f64,
    pub temperature_coefficient: f64,
    pub faiman_heat_loss_constant_w_m2k: f64,
    pub faiman_heat_loss_wind_w_s_m3k: f64,
    pub mounting: crate::model::PhotovoltaicMounting,

}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_pv_system<Op: From<CreatePvSystem>>(index: u32, id: crate::model::EntityId, dc_capacity_w: f64, area_m2: f64, tilt_deg: f64, azimuth_deg: f64, module_efficiency: f64, inverter_efficiency: f64, temperature_coefficient: f64, faiman_heat_loss_constant_w_m2k: f64, faiman_heat_loss_wind_w_s_m3k: f64, mounting: crate::model::PhotovoltaicMounting) -> Op {
    CreatePvSystem { index, id, dc_capacity_w, area_m2, tilt_deg, azimuth_deg, module_efficiency, inverter_efficiency, temperature_coefficient, faiman_heat_loss_constant_w_m2k, faiman_heat_loss_wind_w_s_m3k, mounting }.into()
}
//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod original_creation_tests;
