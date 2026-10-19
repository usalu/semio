//! 🚧️ Energy model mutation — `ChangeSurfaceBoundaryCondition`: Sets what the surface's outside face faces. The tagged union arrives as its two halves — a `newBoundary` discriminator and the `newInterzoneSurfaceId` only the `Interzone` arm carries — and the two must agree.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🚧️ `change-surface-boundary-condition` payload. Sets what the surface's outside face faces. The tagged union arrives as its two halves — a `newBoundary` discriminator and the `newInterzoneSurfaceId` only the `Interzone` arm carries — and the two must agree.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
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

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeSurfaceBoundaryCondition
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeSurfaceBoundaryCondition>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change surface {} boundary to {:?}", self.id.0, self.new_boundary), &format!("Randbedingung von Oberfläche {} auf {:?} ändern", self.id.0, self.new_boundary))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeSurfaceBoundaryCondition
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeSurfaceBoundaryCondition>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "surface", kind: "change-surface-boundary-condition", record: "ChangedSurfaceBoundaryCondition" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
