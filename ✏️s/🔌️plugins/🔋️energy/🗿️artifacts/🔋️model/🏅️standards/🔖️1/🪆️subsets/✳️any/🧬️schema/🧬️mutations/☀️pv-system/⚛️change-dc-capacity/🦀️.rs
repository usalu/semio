//! ⚛️ Energy model mutation — `ChangePvSystemDcCapacity`: Sets DC capacity (W) on one pv system, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚛️ `change-pv-system-dc-capacity` payload. Sets DC capacity (W) on one pv system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-pv-system-dc-capacity")]
pub struct ChangePvSystemDcCapacity {
    pub id: crate::model::EntityId,
    pub new_dc_capacity_w: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_pv_system_dc_capacity<Op: From<ChangePvSystemDcCapacity>>(id: crate::model::EntityId, new_dc_capacity_w: f64) -> Op {
    ChangePvSystemDcCapacity { id, new_dc_capacity_w }.into()
}
//#endregion 🔖️Mutation
