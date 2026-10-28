//! ⛱️ Energy model mutation — `ChangeShadingSurfaceTransmittanceSchedule`: Points the shading surface at one of the model's own schedules for its time-varying solar transmittance, or at none for a fully opaque obstruction.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛱️ `change-shading-surface-transmittance-schedule` payload. Points the shading surface at one of the model's own schedules for its time-varying solar transmittance, or at none for a fully opaque obstruction.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-shading-surface-transmittance-schedule")]
pub struct ChangeShadingSurfaceTransmittanceSchedule {
    pub id: crate::model::EntityId,
    pub new_transmittance_schedule_id: Option<crate::schedule::ScheduleId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_shading_surface_transmittance_schedule<Op: From<ChangeShadingSurfaceTransmittanceSchedule>>(id: crate::model::EntityId, new_transmittance_schedule_id: Option<crate::schedule::ScheduleId>) -> Op {
    ChangeShadingSurfaceTransmittanceSchedule { id, new_transmittance_schedule_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
