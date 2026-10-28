//! 🗻️ Energy model mutation — `ChangeMaterialRoughness`: Sets the surface roughness class of one material, addressed by id.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🗻️ `change-material-roughness` payload. Sets the surface roughness class of one material, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-material-roughness")]
pub struct ChangeMaterialRoughness {
    pub id: crate::model::EntityId,
    pub new_roughness: crate::model::SurfaceRoughness,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_material_roughness<Op: From<ChangeMaterialRoughness>>(id: crate::model::EntityId, new_roughness: crate::model::SurfaceRoughness) -> Op {
    ChangeMaterialRoughness { id, new_roughness }.into()
}
//#endregion 🔖️Mutation
