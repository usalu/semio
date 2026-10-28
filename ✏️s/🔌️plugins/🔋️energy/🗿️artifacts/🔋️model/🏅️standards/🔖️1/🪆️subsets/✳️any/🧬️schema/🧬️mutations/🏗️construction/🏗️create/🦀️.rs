//! 🏗️ Energy model mutation — `CreateConstruction`: Adds one layered construction, outside-to-inside layer order. Every layer must already name a material the document defines, so a construction is never born dangling.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏗️ `create-construction` payload. Adds one layered construction, outside-to-inside layer order. Every layer must already name a material the document defines, so a construction is never born dangling.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-construction")]
pub struct CreateConstruction {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub name: String,
    pub layer_material_ids: Vec<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_construction<Op: From<CreateConstruction>>(index: u32, id: crate::model::EntityId, name: String, layer_material_ids: Vec<crate::model::EntityId>) -> Op {
    CreateConstruction { index, id, name, layer_material_ids }.into()
}
//#endregion 🔖️Mutation
