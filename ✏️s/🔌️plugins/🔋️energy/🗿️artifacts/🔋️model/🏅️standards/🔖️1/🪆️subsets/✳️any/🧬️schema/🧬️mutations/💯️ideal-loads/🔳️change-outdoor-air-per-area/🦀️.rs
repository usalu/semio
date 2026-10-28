//! 🔳️ Energy model mutation — `ChangeIdealLoadsSystemOutdoorAirPerArea`: Sets the ventilation rate the system draws per square metre of floor, in m³/s·m².

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔳️ `change-ideal-loads-system-outdoor-air-per-area` payload. Sets the ventilation rate the system draws per square metre of floor, in m³/s·m².
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-outdoor-air-per-area")]
pub struct ChangeIdealLoadsSystemOutdoorAirPerArea {
    pub id: crate::model::EntityId,
    pub new_outdoor_air_per_area_m3_s_m2: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_ideal_loads_system_outdoor_air_per_area<Op: From<ChangeIdealLoadsSystemOutdoorAirPerArea>>(id: crate::model::EntityId, new_outdoor_air_per_area_m3_s_m2: f64) -> Op {
    ChangeIdealLoadsSystemOutdoorAirPerArea { id, new_outdoor_air_per_area_m3_s_m2 }.into()
}
//#endregion 🔖️Mutation
