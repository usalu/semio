//! 🗂️ Energy model mutation — `ReorderAnnualScheduleRules`: Moves one date rule to another position in the year's rule list. This is the one `reorder` the schedule vocabulary carries, and it earns it: precedence between two rules whose date ranges overlap IS their list order.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🗂️ `reorder-annual-schedule-rules` payload. Moves one date rule to another position in the year's rule list. This is the one `reorder` the schedule vocabulary carries, and it earns it: precedence between two rules whose date ranges overlap IS their list order.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "reorder-annual-schedule-rules")]
pub struct ReorderAnnualScheduleRules {
    pub id: crate::schedule::ScheduleId,
    pub from: u32,
    pub to: u32,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn reorder_annual_schedule_rules<Op: From<ReorderAnnualScheduleRules>>(id: crate::schedule::ScheduleId, from: u32, to: u32) -> Op {
    ReorderAnnualScheduleRules { id, from, to }.into()
}

//#endregion 🔖️Mutation
