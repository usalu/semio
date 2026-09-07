//! 🚧️ Energy model mutation — `ChangeSurfaceBoundaryCondition`: Sets what the surface's outside face faces. The tagged union arrives as its two halves — a `newBoundary` discriminator and the `newInterzoneSurfaceId` only the `Interzone` arm carries — and the two must agree.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚧️ `change-surface-boundary-condition` payload. Sets what the surface's outside face faces. The tagged union arrives as its two halves — a `newBoundary` discriminator and the `newInterzoneSurfaceId` only the `Interzone` arm carries — and the two must agree.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-surface-boundary-condition")]
pub struct ChangeSurfaceBoundaryCondition {
    pub id: crate::model::EntityId,
    pub new_boundary: crate::model::OutsideBoundaryKind,
    pub new_interzone_surface_id: Option<crate::model::EntityId>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_surface_boundary_condition(id: crate::model::EntityId, new_boundary: crate::model::OutsideBoundaryKind, new_interzone_surface_id: Option<crate::model::EntityId>) -> EnergyModelMutation {
    EnergyModelMutation::ChangeSurfaceBoundaryCondition(ChangeSurfaceBoundaryCondition { id, new_boundary, new_interzone_surface_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeSurfaceBoundaryCondition {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "surface", kind: "change-surface-boundary-condition", record: "ChangedSurfaceBoundaryCondition" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change surface {} boundary to {:?}", self.id.0, self.new_boundary)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
