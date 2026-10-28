//! 🐛️ Energy model mutation — `ChangeFaultType`: Sets one fault's degradation mechanism.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🐛️ `change-fault-type` payload. Sets one fault's degradation mechanism.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fault-type")]
pub struct ChangeFaultType {
    pub id: crate::model::EntityId,
    pub new_fault_type: crate::model::FaultType,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fault_type<Op: From<ChangeFaultType>>(id: crate::model::EntityId, new_fault_type: crate::model::FaultType) -> Op {
    ChangeFaultType { id, new_fault_type }.into()
}
//#endregion 🔖️Mutation
