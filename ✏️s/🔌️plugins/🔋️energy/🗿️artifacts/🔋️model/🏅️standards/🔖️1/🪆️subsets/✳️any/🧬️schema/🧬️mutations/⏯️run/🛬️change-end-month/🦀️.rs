//! 🛬️ Energy model mutation — `ChangeRunEndMonth`: Sets the month the simulation run period ends in; the period must stay a calendar interval of its year.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛬️ `change-run-end-month` payload. Sets the month the simulation run period ends in; the period must stay a calendar interval of its year.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-run-end-month")]
pub struct ChangeRunEndMonth {
    pub new_end_month: u8,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_run_end_month<Op: From<ChangeRunEndMonth>>(new_end_month: u8) -> Op {
    ChangeRunEndMonth { new_end_month }.into()
}
//#endregion 🔖️Mutation
