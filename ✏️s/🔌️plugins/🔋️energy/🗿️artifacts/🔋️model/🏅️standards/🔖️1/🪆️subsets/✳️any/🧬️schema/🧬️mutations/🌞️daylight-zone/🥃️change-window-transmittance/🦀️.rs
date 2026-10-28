//! 🥃️ Energy model mutation — `ChangeDaylightZoneWindowTransmittance`: Sets the visible transmittance the daylight control sees its window through, as a fraction.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🥃️ `change-daylight-zone-window-transmittance` payload. Sets the visible transmittance the daylight control sees its window through, as a fraction.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-daylight-zone-window-transmittance")]
pub struct ChangeDaylightZoneWindowTransmittance {
    pub id: crate::model::EntityId,
    pub new_window_transmittance: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_daylight_zone_window_transmittance<Op: From<ChangeDaylightZoneWindowTransmittance>>(id: crate::model::EntityId, new_window_transmittance: f64) -> Op {
    ChangeDaylightZoneWindowTransmittance { id, new_window_transmittance }.into()
}
//#endregion 🔖️Mutation
