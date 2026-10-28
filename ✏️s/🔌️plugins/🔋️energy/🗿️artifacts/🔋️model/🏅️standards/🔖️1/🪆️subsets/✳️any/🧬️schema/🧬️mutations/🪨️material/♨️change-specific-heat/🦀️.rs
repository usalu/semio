//! ♨️ Energy model mutation — `ChangeMaterialSpecificHeat`: Sets specific heat (J/kg·K) on one material, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ♨️ `change-material-specific-heat` payload. Sets specific heat (J/kg·K) on one material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-material-specific-heat")]
pub struct ChangeMaterialSpecificHeat {
    pub id: crate::model::EntityId,
    pub new_specific_heat_j_kg_k: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_material_specific_heat<Op: From<ChangeMaterialSpecificHeat>>(id: crate::model::EntityId, new_specific_heat_j_kg_k: f64) -> Op {
    ChangeMaterialSpecificHeat { id, new_specific_heat_j_kg_k }.into()
}
//#endregion 🔖️Mutation
