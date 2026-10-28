//! ⛏️ Energy model mutation — `ChangeGroundDeep`: Sets the deep ground temperature, read by the ground heat transfer solve.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⛏️ `change-ground-deep` payload. Sets the deep ground temperature, read by the ground heat transfer solve.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ground-deep")]
pub struct ChangeGroundDeep {
    pub new_temperature_c: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_ground_deep<Op: From<ChangeGroundDeep>>(new_temperature_c: f64) -> Op {
    ChangeGroundDeep { new_temperature_c }.into()
}
//#endregion 🔖️Mutation
