//! 🌘️ Energy model mutation — `RemoveElectricalLoadCenterPv`: Detaches one PV system from a load centre; the PV system itself survives, unattached.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌘️ `remove-electrical-load-center-pv` payload. Detaches one PV system from a load centre; the PV system itself survives, unattached.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-electrical-load-center-pv")]
pub struct RemoveElectricalLoadCenterPv {
    pub id: crate::model::EntityId,
    pub pv_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_electrical_load_center_pv<Op: From<RemoveElectricalLoadCenterPv>>(id: crate::model::EntityId, pv_id: crate::model::EntityId) -> Op {
    RemoveElectricalLoadCenterPv { id, pv_id }.into()
}

//#endregion 🔖️Mutation
