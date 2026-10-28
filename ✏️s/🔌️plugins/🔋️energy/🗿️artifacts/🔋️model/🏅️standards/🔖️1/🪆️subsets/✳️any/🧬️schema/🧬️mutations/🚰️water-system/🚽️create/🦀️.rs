//! 🚽️ Energy model mutation — `CreateWaterSystem`: Adds one cold-water use system: the fixtures it serves, their combined peak flow, and the draw schedule the profile is read from.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚽️ `create-water-system` payload. Adds one cold-water use system: the fixtures it serves, their combined peak flow, and the draw schedule the profile is read from.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-water-system")]
pub struct CreateWaterSystem {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub fixture_count: u32,
    pub peak_flow_l_s: f64,
    pub schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_water_system<Op: From<CreateWaterSystem>>(index: u32, id: crate::model::EntityId, fixture_count: u32, peak_flow_l_s: f64, schedule_id: crate::schedule::ScheduleId) -> Op {
    CreateWaterSystem { index, id, fixture_count, peak_flow_l_s, schedule_id }.into()
}
//#endregion 🔖️Mutation
