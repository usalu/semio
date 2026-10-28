//! 🌅️ Energy model mutation — `ChangeSurfaceSunExposed`: Sets whether the outside face receives beam and diffuse solar at all.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌅️ `change-surface-sun-exposed` payload. Sets whether the outside face receives beam and diffuse solar at all.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-sun-exposed")]
pub struct ChangeSurfaceSunExposed {
    pub id: crate::model::EntityId,
    pub new_sun_exposed: bool,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_surface_sun_exposed<Op: From<ChangeSurfaceSunExposed>>(id: crate::model::EntityId, new_sun_exposed: bool) -> Op {
    ChangeSurfaceSunExposed { id, new_sun_exposed }.into()
}
//#endregion 🔖️Mutation
