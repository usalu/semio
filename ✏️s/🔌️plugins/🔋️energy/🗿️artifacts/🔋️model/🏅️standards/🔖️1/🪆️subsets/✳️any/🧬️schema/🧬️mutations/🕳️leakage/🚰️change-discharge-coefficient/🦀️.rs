//! 🚰️ Energy model mutation — `ChangeInfiltrationDischargeCoefficient`: Sets the orifice discharge coefficient on one infiltration, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚰️ `change-infiltration-discharge-coefficient` payload. Sets the orifice discharge coefficient on one infiltration, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-infiltration-discharge-coefficient")]
pub struct ChangeInfiltrationDischargeCoefficient {
    pub id: crate::model::EntityId,
    pub new_discharge_coefficient: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_infiltration_discharge_coefficient<Op: From<ChangeInfiltrationDischargeCoefficient>>(id: crate::model::EntityId, new_discharge_coefficient: f64) -> Op {
    ChangeInfiltrationDischargeCoefficient { id, new_discharge_coefficient }.into()
}
//#endregion 🔖️Mutation
