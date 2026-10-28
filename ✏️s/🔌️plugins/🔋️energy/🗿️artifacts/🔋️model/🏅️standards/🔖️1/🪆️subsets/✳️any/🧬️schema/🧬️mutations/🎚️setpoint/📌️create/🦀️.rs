//! 📌️ Energy model mutation — `CreateSetpointManager`: Creates one loop setpoint manager. `SetpointManagerKind` is a tagged union with no `dsl::DslField`, so the variant travels as its wire name beside the four outdoor-air-reset limits, and every other variant has to carry them as zero.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📌️ `create-setpoint-manager` payload. Creates one loop setpoint manager. `SetpointManagerKind` is a tagged union with no `dsl::DslField`, so the variant travels as its wire name beside the four outdoor-air-reset limits, and every other variant has to carry them as zero.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-setpoint-manager")]
pub struct CreateSetpointManager {
    pub id: crate::model::EntityId,
    pub name: String,
    pub kind: crate::model::SetpointManagerKindTag,
    pub low_outdoor_c: f64,
    pub high_outdoor_c: f64,
    pub low_setpoint_c: f64,
    pub high_setpoint_c: f64,
    pub schedule_present: bool,
    pub schedule_id: crate::schedule::ScheduleId,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_setpoint_manager<Op: From<CreateSetpointManager>>(
    id: crate::model::EntityId,
    name: String,
    kind: crate::model::SetpointManagerKindTag,
    low_outdoor_c: f64,
    high_outdoor_c: f64,
    low_setpoint_c: f64,
    high_setpoint_c: f64,
    schedule_present: bool,
    schedule_id: crate::schedule::ScheduleId,
    index: Option<u32>,
) -> Op {
    CreateSetpointManager { id, name, kind, low_outdoor_c, high_outdoor_c, low_setpoint_c, high_setpoint_c, schedule_present, schedule_id, index }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
