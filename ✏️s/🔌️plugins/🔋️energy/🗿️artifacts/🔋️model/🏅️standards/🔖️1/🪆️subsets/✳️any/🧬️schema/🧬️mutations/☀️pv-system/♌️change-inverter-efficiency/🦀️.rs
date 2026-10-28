//! ♌️ Energy model mutation — `ChangePvSystemInverterEfficiency`: Sets inverter efficiency on one pv system, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ♌️ `change-pv-system-inverter-efficiency` payload. Sets inverter efficiency on one pv system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-pv-system-inverter-efficiency")]
pub struct ChangePvSystemInverterEfficiency {
    pub id: crate::model::EntityId,
    pub new_inverter_efficiency: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_pv_system_inverter_efficiency<Op: From<ChangePvSystemInverterEfficiency>>(id: crate::model::EntityId, new_inverter_efficiency: f64) -> Op {
    ChangePvSystemInverterEfficiency { id, new_inverter_efficiency }.into()
}
//#endregion 🔖️Mutation
