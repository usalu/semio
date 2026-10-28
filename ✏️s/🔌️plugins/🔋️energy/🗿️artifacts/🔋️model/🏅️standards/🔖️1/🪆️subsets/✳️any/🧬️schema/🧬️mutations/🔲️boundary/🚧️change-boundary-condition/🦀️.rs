//! 🚧️ Energy model mutation — `ChangeSurfaceBoundaryCondition`: Sets what the surface's outside face faces. The tagged union arrives as its two halves — a `newBoundary` discriminator and the `newInterzoneSurfaceId` only the `Interzone` arm carries — and the two must agree.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚧️ `change-surface-boundary-condition` payload. Sets what the surface's outside face faces. The tagged union arrives as its two halves — a `newBoundary` discriminator and the `newInterzoneSurfaceId` only the `Interzone` arm carries — and the two must agree.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_s_artifact_energy_model_native_derive::EnergyNativeMutation)]
#[energy_native(schema="🧬️schema/🔣️.json")]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-boundary-condition")]
pub struct ChangeSurfaceBoundaryCondition {
    pub id: crate::model::EntityId,
    pub new_boundary: crate::model::OutsideBoundaryKind,
    pub new_interzone_surface_id: Option<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_surface_boundary_condition<Op: From<ChangeSurfaceBoundaryCondition>>(id: crate::model::EntityId, new_boundary: crate::model::OutsideBoundaryKind, new_interzone_surface_id: Option<crate::model::EntityId>) -> Op {
    ChangeSurfaceBoundaryCondition { id, new_boundary, new_interzone_surface_id }.into()
}

//#endregion 🔖️Mutation

#[cfg(test)]#[path="🧪️native/🦀️.rs"]mod native_tests;
