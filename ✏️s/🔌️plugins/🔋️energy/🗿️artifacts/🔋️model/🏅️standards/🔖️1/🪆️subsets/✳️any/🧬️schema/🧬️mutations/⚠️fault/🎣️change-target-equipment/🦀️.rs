//! 🎣️ Energy model mutation — `ChangeFaultTargetEquipment`: Re-points one fault at another ideal loads system; a target the document does not define is refused.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎣️ `change-fault-target-equipment` payload. Re-points one fault at another ideal loads system; a target the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fault-target-equipment")]
pub struct ChangeFaultTargetEquipment {
    pub id: crate::model::EntityId,
    pub new_target_equipment_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_fault_target_equipment<Op: From<ChangeFaultTargetEquipment>>(id: crate::model::EntityId, new_target_equipment_id: crate::model::EntityId) -> Op {
    ChangeFaultTargetEquipment { id, new_target_equipment_id }.into()
}
//#endregion 🔖️Mutation
