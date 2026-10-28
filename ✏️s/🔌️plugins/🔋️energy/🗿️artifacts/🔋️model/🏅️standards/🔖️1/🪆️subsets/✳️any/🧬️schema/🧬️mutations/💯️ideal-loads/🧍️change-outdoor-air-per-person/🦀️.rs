//! 🧍️ Energy model mutation — `ChangeIdealLoadsSystemOutdoorAirPerPerson`: Sets the ventilation rate the system draws per occupant, in m³/s.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧍️ `change-ideal-loads-system-outdoor-air-per-person` payload. Sets the ventilation rate the system draws per occupant, in m³/s.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-outdoor-air-per-person")]
pub struct ChangeIdealLoadsSystemOutdoorAirPerPerson {
    pub id: crate::model::EntityId,
    pub new_outdoor_air_per_person_m3_s: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_ideal_loads_system_outdoor_air_per_person<Op: From<ChangeIdealLoadsSystemOutdoorAirPerPerson>>(id: crate::model::EntityId, new_outdoor_air_per_person_m3_s: f64) -> Op {
    ChangeIdealLoadsSystemOutdoorAirPerPerson { id, new_outdoor_air_per_person_m3_s }.into()
}
//#endregion 🔖️Mutation
