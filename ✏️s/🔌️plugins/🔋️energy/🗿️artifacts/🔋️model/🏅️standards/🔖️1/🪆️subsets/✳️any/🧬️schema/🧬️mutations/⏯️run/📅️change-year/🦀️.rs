//! 📅️ Energy model mutation — `ChangeRunYear`: Sets the calendar year that fixes the run period's weekdays and leap day; the period must stay a calendar interval of that year.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📅️ `change-run-year` payload. Sets the calendar year that fixes the run period's weekdays and leap day; the period must stay a calendar interval of that year.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-run-year")]
pub struct ChangeRunYear {
    pub new_year: u16,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_run_year<Op: From<ChangeRunYear>>(new_year: u16) -> Op {
    ChangeRunYear { new_year }.into()
}
//#endregion 🔖️Mutation
