//! 🌶️ Energy model mutation — `ChangeFaultSeverity`: Sets severity on one fault, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌶️ `change-fault-severity` payload. Sets severity on one fault, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fault-severity")]
pub struct ChangeFaultSeverity {
    pub id: crate::model::EntityId,
    pub new_severity: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fault_severity<Op: From<ChangeFaultSeverity>>(id: crate::model::EntityId, new_severity: f64) -> Op {
    ChangeFaultSeverity { id, new_severity }.into()
}
//#endregion 🔖️Mutation
