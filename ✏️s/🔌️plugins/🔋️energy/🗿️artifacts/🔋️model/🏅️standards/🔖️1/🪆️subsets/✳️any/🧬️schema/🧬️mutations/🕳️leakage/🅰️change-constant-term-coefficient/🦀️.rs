//! 🅰️ Energy model mutation — `ChangeInfiltrationConstantTermCoefficient`: Sets EnergyPlus's `ZoneInfiltration:DesignFlowRate` constant term coefficient A. The four coefficients are non-negative by that object's own convention (BLAST 0.606/0.03636/0.1177/0, DOE-2 0/0/0.224/0).

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🅰️ `change-infiltration-constant-term-coefficient` payload. Sets EnergyPlus's `ZoneInfiltration:DesignFlowRate` constant term coefficient A. The four coefficients are non-negative by that object's own convention (BLAST 0.606/0.03636/0.1177/0, DOE-2 0/0/0.224/0).
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-constant-term-coefficient")]
pub struct ChangeInfiltrationConstantTermCoefficient {
    pub id: crate::model::EntityId,
    pub new_constant_term_coefficient: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_infiltration_constant_term_coefficient<Op: From<ChangeInfiltrationConstantTermCoefficient>>(id: crate::model::EntityId, new_constant_term_coefficient: f64) -> Op {
    ChangeInfiltrationConstantTermCoefficient { id, new_constant_term_coefficient }.into()
}
//#endregion 🔖️Mutation
