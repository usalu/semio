//! 🔓️ Energy model mutation — `RemoveThermalEnclosureZone`: Takes one zone back out of a thermal enclosure; the zone itself survives.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔓️ `remove-thermal-enclosure-zone` payload. Takes one zone back out of a thermal enclosure; the zone itself survives.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-thermal-enclosure-zone")]
pub struct RemoveThermalEnclosureZone {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_thermal_enclosure_zone<Op: From<RemoveThermalEnclosureZone>>(id: crate::model::EntityId, zone_id: crate::model::EntityId) -> Op {
    RemoveThermalEnclosureZone { id, zone_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
