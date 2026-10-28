//! ❄️ Energy model mutation — `CreateRefrigerationSystem`: Adds one refrigeration system: the display cases it serves, their design load, and the defrost schedule that periodically reverses it.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ❄️ `create-refrigeration-system` payload. Adds one refrigeration system: the display cases it serves, their design load, and the defrost schedule that periodically reverses it.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-refrigeration-system")]
pub struct CreateRefrigerationSystem {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub case_count: u32,
    pub design_load_w: f64,
    pub defrost_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_refrigeration_system<Op: From<CreateRefrigerationSystem>>(index: u32, id: crate::model::EntityId, case_count: u32, design_load_w: f64, defrost_schedule_id: crate::schedule::ScheduleId) -> Op {
    CreateRefrigerationSystem { index, id, case_count, design_load_w, defrost_schedule_id }.into()
}
//#endregion 🔖️Mutation
