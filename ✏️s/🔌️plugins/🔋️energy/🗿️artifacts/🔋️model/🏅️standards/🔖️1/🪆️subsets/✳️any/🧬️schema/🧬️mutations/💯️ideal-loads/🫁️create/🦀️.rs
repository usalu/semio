//! 🫁️ Energy model mutation — `CreateIdealLoadsSystem`: Creates the ideal-loads air system that serves one zone. Both capacity limits are optional — `present` false is the autosized reading, and then the value has to be zero.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🫁️ `create-ideal-loads-system` payload. Creates the ideal-loads air system that serves one zone. Both capacity limits are optional — `present` false is the autosized reading, and then the value has to be zero.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-ideal-loads-system")]
pub struct CreateIdealLoadsSystem {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub max_heating_supply_air_temp_c: f64,
    pub min_cooling_supply_air_temp_c: f64,
    pub max_heating_capacity_present: bool,
    pub max_heating_capacity_w: f64,
    pub max_cooling_capacity_present: bool,
    pub max_cooling_capacity_w: f64,
    pub outdoor_air_per_person_m3_s: f64,
    pub outdoor_air_per_area_m3_s_m2: f64,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_ideal_loads_system<Op: From<CreateIdealLoadsSystem>>(
    id: crate::model::EntityId,
    zone_id: crate::model::EntityId,
    max_heating_supply_air_temp_c: f64,
    min_cooling_supply_air_temp_c: f64,
    max_heating_capacity_present: bool,
    max_heating_capacity_w: f64,
    max_cooling_capacity_present: bool,
    max_cooling_capacity_w: f64,
    outdoor_air_per_person_m3_s: f64,
    outdoor_air_per_area_m3_s_m2: f64,
    index: Option<u32>,
) -> Op {
    CreateIdealLoadsSystem {
        id,
        zone_id,
        max_heating_supply_air_temp_c,
        min_cooling_supply_air_temp_c,
        max_heating_capacity_present,
        max_heating_capacity_w,
        max_cooling_capacity_present,
        max_cooling_capacity_w,
        outdoor_air_per_person_m3_s,
        outdoor_air_per_area_m3_s_m2, index }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
