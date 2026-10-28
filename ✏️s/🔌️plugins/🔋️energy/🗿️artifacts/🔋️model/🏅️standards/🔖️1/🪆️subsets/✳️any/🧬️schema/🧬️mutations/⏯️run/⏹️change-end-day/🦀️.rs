//! ⏹️ Energy model mutation — `ChangeRunEndDay`: Sets the day of month the simulation run period ends on; the period must stay a calendar interval of its year.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⏹️ `change-run-end-day` payload. Sets the day of month the simulation run period ends on; the period must stay a calendar interval of its year.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-run-end-day")]
pub struct ChangeRunEndDay {
    pub new_end_day: u8,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_run_end_day<Op: From<ChangeRunEndDay>>(new_end_day: u8) -> Op {
    ChangeRunEndDay { new_end_day }.into()
}
//#endregion 🔖️Mutation
