//! 🪔️ Energy model mutation — `ChangeDaylightZoneIlluminanceTarget`: Sets the working-plane illuminance the electric lighting dims toward, in lux.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪔️ `change-daylight-zone-illuminance-target` payload. Sets the working-plane illuminance the electric lighting dims toward, in lux.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-daylight-zone-illuminance-target")]
pub struct ChangeDaylightZoneIlluminanceTarget {
    pub id: crate::model::EntityId,
    pub new_illuminance_target_lux: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_daylight_zone_illuminance_target<Op: From<ChangeDaylightZoneIlluminanceTarget>>(id: crate::model::EntityId, new_illuminance_target_lux: f64) -> Op {
    ChangeDaylightZoneIlluminanceTarget { id, new_illuminance_target_lux }.into()
}
//#endregion 🔖️Mutation
