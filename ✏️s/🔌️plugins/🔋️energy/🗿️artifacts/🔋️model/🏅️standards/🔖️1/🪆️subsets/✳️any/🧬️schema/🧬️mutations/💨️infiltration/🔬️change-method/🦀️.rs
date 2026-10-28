//! 🔬️ Energy model mutation — `ChangeInfiltrationMethod`: Selects which of `air_exchange::InfiltrationMethod`'s four flow calculations the kernel runs for one infiltration object. The parameters every method needs are already carried side by side, so switching the method never has to move data.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔬️ `change-infiltration-method` payload. Selects which of `air_exchange::InfiltrationMethod`'s four flow calculations the kernel runs for one infiltration object. The parameters every method needs are already carried side by side, so switching the method never has to move data.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-method")]
pub struct ChangeInfiltrationMethod {
    pub id: crate::model::EntityId,
    pub new_method: crate::air_exchange::InfiltrationMethod,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_infiltration_method<Op: From<ChangeInfiltrationMethod>>(id: crate::model::EntityId, new_method: crate::air_exchange::InfiltrationMethod) -> Op {
    ChangeInfiltrationMethod { id, new_method }.into()
}
//#endregion 🔖️Mutation
