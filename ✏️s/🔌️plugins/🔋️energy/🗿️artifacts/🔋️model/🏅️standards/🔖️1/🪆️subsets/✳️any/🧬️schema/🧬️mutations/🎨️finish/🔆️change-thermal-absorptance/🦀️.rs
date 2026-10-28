//! 🔆️ Energy model mutation — `ChangeMaterialThermalAbsorptance`: Sets thermal absorptance on one material, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔆️ `change-material-thermal-absorptance` payload. Sets thermal absorptance on one material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-material-thermal-absorptance")]
pub struct ChangeMaterialThermalAbsorptance {
    pub id: crate::model::EntityId,
    pub new_thermal_absorptance: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_material_thermal_absorptance<Op: From<ChangeMaterialThermalAbsorptance>>(id: crate::model::EntityId, new_thermal_absorptance: f64) -> Op {
    ChangeMaterialThermalAbsorptance { id, new_thermal_absorptance }.into()
}
//#endregion 🔖️Mutation
