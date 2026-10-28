//! 🌍️ Energy model mutation — `ChangeSiteLatitude`: Sets the site's geographic latitude, north positive.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌍️ `change-site-latitude` payload. Sets the site's geographic latitude, north positive.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-site-latitude")]
pub struct ChangeSiteLatitude {
    pub new_latitude_deg: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_site_latitude<Op: From<ChangeSiteLatitude>>(new_latitude_deg: f64) -> Op {
    ChangeSiteLatitude { new_latitude_deg }.into()
}
//#endregion 🔖️Mutation
