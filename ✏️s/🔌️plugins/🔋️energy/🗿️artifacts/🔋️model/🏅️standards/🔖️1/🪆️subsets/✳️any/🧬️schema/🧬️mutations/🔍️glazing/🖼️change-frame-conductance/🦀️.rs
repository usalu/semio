//! 🖼️ Energy model mutation — `ChangeFenestrationFrameConductance`: Sets the frame's whole-assembly thermal conductance in W/K, added to the glazing term rather than distributed over the area.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🖼️ `change-fenestration-frame-conductance` payload. Sets the frame's whole-assembly thermal conductance in W/K, added to the glazing term rather than distributed over the area.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-frame-conductance")]
pub struct ChangeFenestrationFrameConductance {
    pub id: crate::model::EntityId,
    pub new_frame_conductance_w_k: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_frame_conductance<Op: From<ChangeFenestrationFrameConductance>>(id: crate::model::EntityId, new_frame_conductance_w_k: f64) -> Op {
    ChangeFenestrationFrameConductance { id, new_frame_conductance_w_k }.into()
}
//#endregion 🔖️Mutation
