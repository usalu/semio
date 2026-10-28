//! 🕶️ Energy model mutation — `ChangeDaylightZoneGlareLimit`: Sets the maximum discomfort glare index the control tolerates.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕶️ `change-daylight-zone-glare-limit` payload. Sets the maximum discomfort glare index the control tolerates.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-daylight-zone-glare-limit")]
pub struct ChangeDaylightZoneGlareLimit {
    pub id: crate::model::EntityId,
    pub new_glare_limit: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_daylight_zone_glare_limit<Op: From<ChangeDaylightZoneGlareLimit>>(id: crate::model::EntityId, new_glare_limit: f64) -> Op {
    ChangeDaylightZoneGlareLimit { id, new_glare_limit }.into()
}
//#endregion 🔖️Mutation
