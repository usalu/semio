//! 🪣️ Energy model mutation — `ChangeWaterSystemFixtureCount`: Sets how many fixtures one water system carries.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪣️ `change-water-system-fixture-count` payload. Sets how many fixtures one water system carries.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-water-system-fixture-count")]
pub struct ChangeWaterSystemFixtureCount {
    pub id: crate::model::EntityId,
    pub new_fixture_count: u32,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_water_system_fixture_count<Op: From<ChangeWaterSystemFixtureCount>>(id: crate::model::EntityId, new_fixture_count: u32) -> Op {
    ChangeWaterSystemFixtureCount { id, new_fixture_count }.into()
}
//#endregion 🔖️Mutation
