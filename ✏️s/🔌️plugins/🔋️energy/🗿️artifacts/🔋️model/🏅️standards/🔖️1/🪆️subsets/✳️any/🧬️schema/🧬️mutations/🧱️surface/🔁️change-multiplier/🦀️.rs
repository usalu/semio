//! 🔁️ Energy model mutation — `ChangeSurfaceMultiplier`: Sets how many identical instances of the surface the heat balance is scaled by.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔁️ `change-surface-multiplier` payload. Sets how many identical instances of the surface the heat balance is scaled by.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-multiplier")]
pub struct ChangeSurfaceMultiplier {
    pub id: crate::model::EntityId,
    pub new_multiplier: u32,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_surface_multiplier<Op: From<ChangeSurfaceMultiplier>>(id: crate::model::EntityId, new_multiplier: u32) -> Op {
    ChangeSurfaceMultiplier { id, new_multiplier }.into()
}
//#endregion 🔖️Mutation
