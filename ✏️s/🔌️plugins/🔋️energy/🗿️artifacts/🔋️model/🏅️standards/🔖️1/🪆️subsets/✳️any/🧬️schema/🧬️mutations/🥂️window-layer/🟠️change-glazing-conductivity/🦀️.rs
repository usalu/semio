//! 🟠️ Energy model mutation — `ChangeGlazingMaterialConductivity`: Sets pane conductivity (W/m·K) on one glazing material, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟠️ `change-glazing-material-conductivity` payload. Sets pane conductivity (W/m·K) on one glazing material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-glazing-material-conductivity")]
pub struct ChangeGlazingMaterialConductivity {
    pub id: crate::model::EntityId,
    pub new_conductivity_w_m_k: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_glazing_material_conductivity<Op: From<ChangeGlazingMaterialConductivity>>(id: crate::model::EntityId, new_conductivity_w_m_k: f64) -> Op {
    ChangeGlazingMaterialConductivity { id, new_conductivity_w_m_k }.into()
}
//#endregion 🔖️Mutation
