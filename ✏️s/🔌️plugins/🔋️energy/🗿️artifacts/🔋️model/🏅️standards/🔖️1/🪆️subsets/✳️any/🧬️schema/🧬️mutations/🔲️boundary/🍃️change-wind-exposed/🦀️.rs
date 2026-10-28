//! 🍃️ Energy model mutation — `ChangeSurfaceWindExposed`: Sets whether the outside face uses the wind-driven exterior convection correlation or the sheltered one.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🍃️ `change-surface-wind-exposed` payload. Sets whether the outside face uses the wind-driven exterior convection correlation or the sheltered one.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-wind-exposed")]
pub struct ChangeSurfaceWindExposed {
    pub id: crate::model::EntityId,
    pub new_wind_exposed: bool,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_surface_wind_exposed<Op: From<ChangeSurfaceWindExposed>>(id: crate::model::EntityId, new_wind_exposed: bool) -> Op {
    ChangeSurfaceWindExposed { id, new_wind_exposed }.into()
}
//#endregion 🔖️Mutation
