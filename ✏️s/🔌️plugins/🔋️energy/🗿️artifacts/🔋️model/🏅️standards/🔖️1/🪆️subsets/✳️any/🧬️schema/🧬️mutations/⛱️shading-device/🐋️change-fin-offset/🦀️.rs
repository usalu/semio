//! 🐋️ Energy model mutation — `ChangeFenestrationFinOffset`: Sets how far beside the window jambs the two vertical projections stand, in metres.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🐋️ `change-fenestration-fin-offset` payload. Sets how far beside the window jambs the two vertical projections stand, in metres.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-fin-offset")]
pub struct ChangeFenestrationFinOffset {
    pub id: crate::model::EntityId,
    pub new_fin_offset_m: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_fin_offset<Op: From<ChangeFenestrationFinOffset>>(id: crate::model::EntityId, new_fin_offset_m: f64) -> Op {
    ChangeFenestrationFinOffset { id, new_fin_offset_m }.into()
}
//#endregion 🔖️Mutation
