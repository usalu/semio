//! 📦️ Energy model mutation — `ChangeZoneVolume`: Sets one zone's air volume in cubic metres — the capacitance the zone air heat balance integrates over.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📦️ `change-zone-volume` payload. Sets one zone's air volume in cubic metres — the capacitance the zone air heat balance integrates over.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-volume")]
pub struct ChangeZoneVolume {
    pub id: crate::model::EntityId,
    pub new_volume_m3: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_zone_volume<Op: From<ChangeZoneVolume>>(id: crate::model::EntityId, new_volume_m3: f64) -> Op {
    ChangeZoneVolume { id, new_volume_m3 }.into()
}
//#endregion 🔖️Mutation
