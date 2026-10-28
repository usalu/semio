//! 🧊️ Energy model mutation — `ChangePvSystemFaimanHeatLossConstant`: Sets Constant heat loss on one pv system, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧊️ `change-pv-system-faiman-heat-loss-constant` payload. Sets Constant heat loss on one pv system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-pv-system-faiman-heat-loss-constant")]
pub struct ChangePvSystemFaimanHeatLossConstant {
    pub id: crate::model::EntityId,
    pub new_faiman_heat_loss_constant_w_m2k: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_pv_system_faiman_heat_loss_constant<Op: From<ChangePvSystemFaimanHeatLossConstant>>(id: crate::model::EntityId, new_faiman_heat_loss_constant_w_m2k: f64) -> Op {
    ChangePvSystemFaimanHeatLossConstant { id, new_faiman_heat_loss_constant_w_m2k }.into()
}
//#endregion 🔖️Mutation
