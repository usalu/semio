//! 🅱️ Energy model mutation — `ChangeInfiltrationTemperatureTermCoefficient`: Sets the temperature term coefficient B on one infiltration, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🅱️ `change-infiltration-temperature-term-coefficient` payload. Sets the temperature term coefficient B on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-temperature-term-coefficient")]
pub struct ChangeInfiltrationTemperatureTermCoefficient {
    pub id: crate::model::EntityId,
    pub new_temperature_term_coefficient: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_infiltration_temperature_term_coefficient<Op: From<ChangeInfiltrationTemperatureTermCoefficient>>(id: crate::model::EntityId, new_temperature_term_coefficient: f64) -> Op {
    ChangeInfiltrationTemperatureTermCoefficient { id, new_temperature_term_coefficient }.into()
}
//#endregion 🔖️Mutation
