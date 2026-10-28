//! 🌐️ Energy model mutation — `ChangeFenestrationUValue`: Sets the glazing's air-to-air thermal transmittance in W/(m²·K) — the conductance the film-free window term multiplies by area and the outdoor-to-zone temperature difference.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌐️ `change-fenestration-u-value` payload. Sets the glazing's air-to-air thermal transmittance in W/(m²·K) — the conductance the film-free window term multiplies by area and the outdoor-to-zone temperature difference.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fenestration-u-value")]
pub struct ChangeFenestrationUValue {
    pub id: crate::model::EntityId,
    pub new_u_value_w_m2k: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fenestration_u_value<Op: From<ChangeFenestrationUValue>>(id: crate::model::EntityId, new_u_value_w_m2k: f64) -> Op {
    ChangeFenestrationUValue { id, new_u_value_w_m2k }.into()
}
//#endregion 🔖️Mutation
