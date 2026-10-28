//! ⛓️ Energy model mutation — `ChangeOutdoorAirSystemAirLoop`: Moves the outdoor air system to another air loop.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛓️ `change-outdoor-air-system-air-loop` payload. Moves the outdoor air system to another air loop.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-outdoor-air-system-air-loop")]
pub struct ChangeOutdoorAirSystemAirLoop {
    pub id: crate::model::EntityId,
    pub new_air_loop_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_outdoor_air_system_air_loop<Op: From<ChangeOutdoorAirSystemAirLoop>>(id: crate::model::EntityId, new_air_loop_id: crate::model::EntityId) -> Op {
    ChangeOutdoorAirSystemAirLoop { id, new_air_loop_id }.into()
}
//#endregion 🔖️Mutation
