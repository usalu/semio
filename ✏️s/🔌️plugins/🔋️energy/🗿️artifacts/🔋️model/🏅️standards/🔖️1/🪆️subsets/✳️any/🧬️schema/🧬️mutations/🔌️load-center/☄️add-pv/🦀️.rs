//! ☄️ Energy model mutation — `AddElectricalLoadCenterPv`: Attaches one PV system to a load centre, so its DC output is inverted onto that centre's bus.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ☄️ `add-electrical-load-center-pv` payload. Attaches one PV system to a load centre, so its DC output is inverted onto that centre's bus.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-electrical-load-center-pv")]
pub struct AddElectricalLoadCenterPv {
    pub id: crate::model::EntityId,
    pub index: u32,
    pub pv_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn add_electrical_load_center_pv<Op: From<AddElectricalLoadCenterPv>>(id: crate::model::EntityId, index: u32, pv_id: crate::model::EntityId) -> Op {
    AddElectricalLoadCenterPv { id, index, pv_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
