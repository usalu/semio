//! 🪝️ Energy model mutation — `RemoveElectricalLoadCenterBattery`: Detaches one battery from a load centre; the battery itself survives, unattached.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪝️ `remove-electrical-load-center-battery` payload. Detaches one battery from a load centre; the battery itself survives, unattached.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-electrical-load-center-battery")]
pub struct RemoveElectricalLoadCenterBattery {
    pub id: crate::model::EntityId,
    pub battery_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_electrical_load_center_battery<Op: From<RemoveElectricalLoadCenterBattery>>(id: crate::model::EntityId, battery_id: crate::model::EntityId) -> Op {
    RemoveElectricalLoadCenterBattery { id, battery_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
