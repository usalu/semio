//! 🎼️ Energy model mutation — `ChangeSetpointManagerSchedule`: Points one setpoint manager at a schedule the model defines, or clears the slot. `Option<T>` has no `dsl::DslField`, so the optional reference travels as a `present` flag beside the id and an absent slot has to carry zero.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎼️ `change-setpoint-manager-schedule` payload. Points one setpoint manager at a schedule the model defines, or clears the slot. `Option<T>` has no `dsl::DslField`, so the optional reference travels as a `present` flag beside the id and an absent slot has to carry zero.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-setpoint-manager-schedule")]
pub struct ChangeSetpointManagerSchedule {
    pub id: crate::model::EntityId,
    pub new_schedule_present: bool,
    pub new_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_setpoint_manager_schedule<Op: From<ChangeSetpointManagerSchedule>>(id: crate::model::EntityId, new_schedule_present: bool, new_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeSetpointManagerSchedule { id, new_schedule_present, new_schedule_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
