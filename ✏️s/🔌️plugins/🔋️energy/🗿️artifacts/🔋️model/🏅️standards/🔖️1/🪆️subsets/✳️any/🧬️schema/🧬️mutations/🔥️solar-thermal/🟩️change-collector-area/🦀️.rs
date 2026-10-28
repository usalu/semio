//! 🟩️ Energy model mutation — `ChangeSolarThermalSystemCollectorArea`: Sets collector area (m²) on one solar thermal system, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟩️ `change-solar-thermal-system-collector-area` payload. Sets collector area (m²) on one solar thermal system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-solar-thermal-system-collector-area")]
pub struct ChangeSolarThermalSystemCollectorArea {
    pub id: crate::model::EntityId,
    pub new_collector_area_m2: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_solar_thermal_system_collector_area<Op: From<ChangeSolarThermalSystemCollectorArea>>(id: crate::model::EntityId, new_collector_area_m2: f64) -> Op {
    ChangeSolarThermalSystemCollectorArea { id, new_collector_area_m2 }.into()
}
//#endregion 🔖️Mutation
