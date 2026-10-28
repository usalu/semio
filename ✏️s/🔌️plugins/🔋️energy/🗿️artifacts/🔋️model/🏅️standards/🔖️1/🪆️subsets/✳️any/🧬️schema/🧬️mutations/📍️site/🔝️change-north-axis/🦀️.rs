//! 🔝️ Energy model mutation — `ChangeSiteNorthAxis`: Sets the angle from true north to the building's north axis, clockwise.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔝️ `change-site-north-axis` payload. Sets the angle from true north to the building's north axis, clockwise.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-site-north-axis")]
pub struct ChangeSiteNorthAxis {
    pub new_north_axis_deg: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_site_north_axis<Op: From<ChangeSiteNorthAxis>>(new_north_axis_deg: f64) -> Op {
    ChangeSiteNorthAxis { new_north_axis_deg }.into()
}
//#endregion 🔖️Mutation
