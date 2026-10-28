//! ⚠️ Energy model mutation — `CreateFault`: Adds one equipment fault. `targetEquipmentId` is checked against `ideal_loads` because that is the collection the kernel actually matches it against (`SystemSubstepStage::Fault` compares `fault.target_equipment_id == ideal.id`); the field's own type carries no discriminator, so this is the only referent the engine gives it.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚠️ `create-fault` payload. Adds one equipment fault. `targetEquipmentId` is checked against `ideal_loads` because that is the collection the kernel actually matches it against (`SystemSubstepStage::Fault` compares `fault.target_equipment_id == ideal.id`); the field's own type carries no discriminator, so this is the only referent the engine gives it.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-fault")]
pub struct CreateFault {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub target_equipment_id: crate::model::EntityId,
    pub fault_type: crate::model::FaultType,
    pub severity: f64,
    pub start_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_fault<Op: From<CreateFault>>(index: u32, id: crate::model::EntityId, target_equipment_id: crate::model::EntityId, fault_type: crate::model::FaultType, severity: f64, start_schedule_id: crate::schedule::ScheduleId) -> Op {
    CreateFault { index, id, target_equipment_id, fault_type, severity, start_schedule_id }.into()
}
//#endregion 🔖️Mutation
