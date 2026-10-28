//! 🟧️ Energy model mutation — `ChangeIdealLoadsSystemMaxCoolingCapacity`: Sets or clears the ideal-loads cooling capacity limit in watts. Cleared means autosized.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🟧️ `change-ideal-loads-system-max-cooling-capacity` payload. Sets or clears the ideal-loads cooling capacity limit in watts. Cleared means autosized.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-ideal-loads-system-max-cooling-capacity")]
pub struct ChangeIdealLoadsSystemMaxCoolingCapacity {
    pub id: crate::model::EntityId,
    pub new_capacity_present: bool,
    pub new_max_cooling_capacity_w: f64,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_ideal_loads_system_max_cooling_capacity<Op: From<ChangeIdealLoadsSystemMaxCoolingCapacity>>(id: crate::model::EntityId, new_capacity_present: bool, new_max_cooling_capacity_w: f64) -> Op {
    ChangeIdealLoadsSystemMaxCoolingCapacity { id, new_capacity_present, new_max_cooling_capacity_w }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
