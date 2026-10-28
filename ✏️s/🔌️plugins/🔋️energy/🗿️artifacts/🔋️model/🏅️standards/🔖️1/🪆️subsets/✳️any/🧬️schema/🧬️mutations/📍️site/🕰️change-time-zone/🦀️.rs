//! 🕰️ Energy model mutation — `ChangeSiteTimeZone`: Sets the site's standard time zone as an offset from UTC.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕰️ `change-site-time-zone` payload. Sets the site's standard time zone as an offset from UTC.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-site-time-zone")]
pub struct ChangeSiteTimeZone {
    pub new_time_zone_hours: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_site_time_zone<Op: From<ChangeSiteTimeZone>>(new_time_zone_hours: f64) -> Op {
    ChangeSiteTimeZone { new_time_zone_hours }.into()
}
//#endregion 🔖️Mutation
