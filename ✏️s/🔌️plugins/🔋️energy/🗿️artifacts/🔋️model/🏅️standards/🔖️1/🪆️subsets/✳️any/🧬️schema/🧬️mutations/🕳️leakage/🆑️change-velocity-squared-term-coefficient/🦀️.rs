//! 🆑️ Energy model mutation — `ChangeInfiltrationVelocitySquaredTermCoefficient`: Sets the squared wind velocity term coefficient D on one infiltration, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🆑️ `change-infiltration-velocity-squared-term-coefficient` payload. Sets the squared wind velocity term coefficient D on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-velocity-squared-term-coefficient")]
pub struct ChangeInfiltrationVelocitySquaredTermCoefficient {
    pub id: crate::model::EntityId,
    pub new_velocity_squared_term_coefficient: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_infiltration_velocity_squared_term_coefficient<Op: From<ChangeInfiltrationVelocitySquaredTermCoefficient>>(id: crate::model::EntityId, new_velocity_squared_term_coefficient: f64) -> Op {
    ChangeInfiltrationVelocitySquaredTermCoefficient { id, new_velocity_squared_term_coefficient }.into()
}
//#endregion 🔖️Mutation
