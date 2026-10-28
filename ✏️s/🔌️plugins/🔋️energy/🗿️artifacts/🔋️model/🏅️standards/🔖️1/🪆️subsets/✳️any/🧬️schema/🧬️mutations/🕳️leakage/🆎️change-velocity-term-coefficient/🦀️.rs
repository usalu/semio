//! 🆎️ Energy model mutation — `ChangeInfiltrationVelocityTermCoefficient`: Sets the wind velocity term coefficient C on one infiltration, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🆎️ `change-infiltration-velocity-term-coefficient` payload. Sets the wind velocity term coefficient C on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-velocity-term-coefficient")]
pub struct ChangeInfiltrationVelocityTermCoefficient {
    pub id: crate::model::EntityId,
    pub new_velocity_term_coefficient: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_infiltration_velocity_term_coefficient<Op: From<ChangeInfiltrationVelocityTermCoefficient>>(id: crate::model::EntityId, new_velocity_term_coefficient: f64) -> Op {
    ChangeInfiltrationVelocityTermCoefficient { id, new_velocity_term_coefficient }.into()
}
//#endregion 🔖️Mutation
