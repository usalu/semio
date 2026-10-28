//! ⛰️ Energy model mutation — `ChangeSiteElevation`: Sets the site's elevation above sea level (EnergyPlus `Site:Location`: -300 m to below 8900 m).

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛰️ `change-site-elevation` payload. Sets the site's elevation above sea level (EnergyPlus `Site:Location`: -300 m to below 8900 m).
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-site-elevation")]
pub struct ChangeSiteElevation {
    pub new_elevation_m: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_site_elevation<Op: From<ChangeSiteElevation>>(new_elevation_m: f64) -> Op {
    ChangeSiteElevation { new_elevation_m }.into()
}
//#endregion 🔖️Mutation
