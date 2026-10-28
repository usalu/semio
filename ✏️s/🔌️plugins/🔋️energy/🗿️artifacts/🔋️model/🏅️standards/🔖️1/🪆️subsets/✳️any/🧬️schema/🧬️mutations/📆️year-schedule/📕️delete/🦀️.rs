//! 📕️ Energy model mutation — `DeleteAnnualSchedule`: Removes one rule-based year. Refused while any consumer still resolves its id. Its inverse is a cascade: the create that re-defines the year, then one insert per date rule in order, then one add per holiday — because the rules and the holidays are collections of their own and the create does not carry them.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📕️ `delete-annual-schedule` payload. Removes one rule-based year. Refused while any consumer still resolves its id. Its inverse is a cascade: the create that re-defines the year, then one insert per date rule in order, then one add per holiday — because the rules and the holidays are collections of their own and the create does not carry them.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-annual-schedule")]
pub struct DeleteAnnualSchedule {
    pub id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_annual_schedule<Op: From<DeleteAnnualSchedule>>(id: crate::schedule::ScheduleId) -> Op {
    DeleteAnnualSchedule { id }.into()
}

//#endregion 🔖️Mutation
