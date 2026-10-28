//! ▶️ Energy model mutation — `ChangeRunStartDay`: Sets the day of month the simulation run period starts on; the period must stay a calendar interval of its year.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ▶️ `change-run-start-day` payload. Sets the day of month the simulation run period starts on; the period must stay a calendar interval of its year.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-run-start-day")]
pub struct ChangeRunStartDay {
    pub new_start_day: u8,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_run_start_day<Op: From<ChangeRunStartDay>>(new_start_day: u8) -> Op {
    ChangeRunStartDay { new_start_day }.into()
}
//#endregion 🔖️Mutation
