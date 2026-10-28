//! 🧩️ Energy model mutation — `ChangeSurfaceClass`: Sets which of the eight boundary roles the surface plays — the role the kernel reads to decide whether it sees sky, ground or another zone.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧩️ `change-surface-class` payload. Sets which of the eight boundary roles the surface plays — the role the kernel reads to decide whether it sees sky, ground or another zone.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-class")]
pub struct ChangeSurfaceClass {
    pub id: crate::model::EntityId,
    pub new_class: crate::model::SurfaceClass,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_surface_class<Op: From<ChangeSurfaceClass>>(id: crate::model::EntityId, new_class: crate::model::SurfaceClass) -> Op {
    ChangeSurfaceClass { id, new_class }.into()
}
//#endregion 🔖️Mutation
