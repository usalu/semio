//! 📙️ Energy model mutation — `RemoveAnnualScheduleRule`: Takes one date rule out of a year's ordered rule list, addressed by its BASE-state index. Every later rule moves up one, and a day the removed rule used to answer falls through to the next matching rule or to the default profile.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📙️ `remove-annual-schedule-rule` payload. Takes one date rule out of a year's ordered rule list, addressed by its BASE-state index. Every later rule moves up one, and a day the removed rule used to answer falls through to the next matching rule or to the default profile.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-annual-schedule-rule")]
pub struct RemoveAnnualScheduleRule {
    pub id: crate::schedule::ScheduleId,
    pub index: u32,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_annual_schedule_rule<Op: From<RemoveAnnualScheduleRule>>(id: crate::schedule::ScheduleId, index: u32) -> Op {
    RemoveAnnualScheduleRule { id, index }.into()
}

//#endregion 🔖️Mutation
