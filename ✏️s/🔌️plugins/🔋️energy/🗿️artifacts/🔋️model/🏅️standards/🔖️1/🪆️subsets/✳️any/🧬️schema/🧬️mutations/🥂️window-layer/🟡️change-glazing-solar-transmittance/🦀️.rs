//! 🟡️ Energy model mutation — `ChangeGlazingMaterialSolarTransmittance`: Sets normal-incidence solar transmittance on one glazing material, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟡️ `change-glazing-material-solar-transmittance` payload. Sets normal-incidence solar transmittance on one glazing material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-glazing-material-solar-transmittance")]
pub struct ChangeGlazingMaterialSolarTransmittance {
    pub id: crate::model::EntityId,
    pub new_solar_transmittance: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_glazing_material_solar_transmittance<Op: From<ChangeGlazingMaterialSolarTransmittance>>(id: crate::model::EntityId, new_solar_transmittance: f64) -> Op {
    ChangeGlazingMaterialSolarTransmittance { id, new_solar_transmittance }.into()
}
//#endregion 🔖️Mutation
