//! 📈️ Energy model mutation — `ChangePvSystemTilt`: Sets tilt on one pv system, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📈️ `change-pv-system-tilt` payload. Sets tilt on one pv system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-pv-system-tilt")]
pub struct ChangePvSystemTilt {
    pub id: crate::model::EntityId,
    pub new_tilt_deg: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_pv_system_tilt<Op: From<ChangePvSystemTilt>>(id: crate::model::EntityId, new_tilt_deg: f64) -> Op {
    ChangePvSystemTilt { id, new_tilt_deg }.into()
}
//#endregion 🔖️Mutation
