//! 🏹️ Energy model mutation — `ChangeShwSystemSetpoint`: Sets tank setpoint (°C) on one service hot water system, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏹️ `change-shw-system-setpoint` payload. Sets tank setpoint (°C) on one service hot water system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-shw-system-setpoint")]
pub struct ChangeShwSystemSetpoint {
    pub id: crate::model::EntityId,
    pub new_setpoint_c: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_shw_system_setpoint<Op: From<ChangeShwSystemSetpoint>>(id: crate::model::EntityId, new_setpoint_c: f64) -> Op {
    ChangeShwSystemSetpoint { id, new_setpoint_c }.into()
}
//#endregion 🔖️Mutation
