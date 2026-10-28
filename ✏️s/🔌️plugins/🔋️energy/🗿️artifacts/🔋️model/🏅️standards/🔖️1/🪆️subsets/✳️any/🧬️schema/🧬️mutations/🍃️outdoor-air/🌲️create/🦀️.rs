//! 🌲️ Energy model mutation — `CreateOutdoorAirSystem`: Puts an outdoor air system on one air loop: the minimum outdoor air flow it always draws and whether its economizer may open beyond that.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌲️ `create-outdoor-air-system` payload. Puts an outdoor air system on one air loop: the minimum outdoor air flow it always draws and whether its economizer may open beyond that.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-outdoor-air-system")]
pub struct CreateOutdoorAirSystem {
    pub id: crate::model::EntityId,
    pub air_loop_id: crate::model::EntityId,
    pub min_oa_flow_m3_s: f64,
    pub economizer_enabled: bool,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_outdoor_air_system<Op: From<CreateOutdoorAirSystem>>(id: crate::model::EntityId, air_loop_id: crate::model::EntityId, min_oa_flow_m3_s: f64, economizer_enabled: bool, index: Option<u32>) -> Op {
    CreateOutdoorAirSystem { id, air_loop_id, min_oa_flow_m3_s, economizer_enabled, index }.into()
}
//#endregion 🔖️Mutation
