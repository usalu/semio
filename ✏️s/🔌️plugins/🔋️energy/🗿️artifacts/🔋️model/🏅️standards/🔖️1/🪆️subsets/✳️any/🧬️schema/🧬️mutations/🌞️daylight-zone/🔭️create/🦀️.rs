//! 🔭️ Energy model mutation — `CreateDaylightZone`: Puts a daylighting control on one zone: an illuminance target the electric lighting dims toward, a glare index limit and the visible transmittance the control sees the window through.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔭️ `create-daylight-zone` payload. Puts a daylighting control on one zone: an illuminance target the electric lighting dims toward, a glare index limit and the visible transmittance the control sees the window through.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeRowBirth, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[energy_native_birth(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-daylight-zone")]
pub struct CreateDaylightZone {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub illuminance_target_lux: f64,
    pub glare_limit: f64,
    pub window_transmittance: f64,
    pub index: Option<u32>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_daylight_zone<Op: From<CreateDaylightZone>>(id: crate::model::EntityId, zone_id: crate::model::EntityId, illuminance_target_lux: f64, glare_limit: f64, window_transmittance: f64, index: Option<u32>) -> Op {
    CreateDaylightZone { id, zone_id, illuminance_target_lux, glare_limit, window_transmittance, index }.into()
}
//#endregion 🔖️Mutation
