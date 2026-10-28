//! 🟨️ Energy model mutation — `ChangePvSystemArea`: Sets aperture area (m²) on one pv system, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟨️ `change-pv-system-area` payload. Sets aperture area (m²) on one pv system, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-pv-system-area")]
pub struct ChangePvSystemArea {
    pub id: crate::model::EntityId,
    pub new_area_m2: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_pv_system_area<Op: From<ChangePvSystemArea>>(id: crate::model::EntityId, new_area_m2: f64) -> Op {
    ChangePvSystemArea { id, new_area_m2 }.into()
}
//#endregion 🔖️Mutation
