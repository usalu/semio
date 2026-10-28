//! 🛫️ Energy model mutation — `ChangeRunStartMonth`: Sets the month the simulation run period starts in; the period must stay a calendar interval of its year.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛫️ `change-run-start-month` payload. Sets the month the simulation run period starts in; the period must stay a calendar interval of its year.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-run-start-month")]
pub struct ChangeRunStartMonth {
    pub new_start_month: u8,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_run_start_month<Op: From<ChangeRunStartMonth>>(new_start_month: u8) -> Op {
    ChangeRunStartMonth { new_start_month }.into()
}
//#endregion 🔖️Mutation
