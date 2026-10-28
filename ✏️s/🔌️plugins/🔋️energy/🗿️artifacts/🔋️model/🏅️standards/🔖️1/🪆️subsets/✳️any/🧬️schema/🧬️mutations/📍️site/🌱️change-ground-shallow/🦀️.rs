//! 🌱️ Energy model mutation — `ChangeGroundShallow`: Sets one month's shallow ground temperature, read by the ground heat transfer solve.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌱️ `change-ground-shallow` payload. Sets one month's shallow ground temperature, read by the ground heat transfer solve.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ground-shallow")]
pub struct ChangeGroundShallow {
    pub month: u8,
    pub new_temperature_c: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_ground_shallow<Op: From<ChangeGroundShallow>>(month: u8, new_temperature_c: f64) -> Op {
    ChangeGroundShallow { month, new_temperature_c }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
