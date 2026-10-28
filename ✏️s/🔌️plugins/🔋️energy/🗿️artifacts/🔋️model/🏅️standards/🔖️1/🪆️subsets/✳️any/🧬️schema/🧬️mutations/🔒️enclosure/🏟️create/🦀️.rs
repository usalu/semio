//! 🏟️ Energy model mutation — `CreateThermalEnclosure`: Adds one thermal enclosure — the named set of zones that share one continuous envelope boundary, which is what an envelope-area report normalizes over.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏟️ `create-thermal-enclosure` payload. Adds one thermal enclosure — the named set of zones that share one continuous envelope boundary, which is what an envelope-area report normalizes over.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-thermal-enclosure")]
pub struct CreateThermalEnclosure {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub name: String,
    pub zone_ids: Vec<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_thermal_enclosure<Op: From<CreateThermalEnclosure>>(index: u32, id: crate::model::EntityId, name: String, zone_ids: Vec<crate::model::EntityId>) -> Op {
    CreateThermalEnclosure { index, id, name, zone_ids }.into()
}
//#endregion 🔖️Mutation
