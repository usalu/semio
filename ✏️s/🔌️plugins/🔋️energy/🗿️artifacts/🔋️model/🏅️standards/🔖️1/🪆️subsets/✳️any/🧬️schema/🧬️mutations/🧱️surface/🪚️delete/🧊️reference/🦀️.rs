//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::DeleteSurface;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for DeleteSurface
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<DeleteSurface> + From<crate::boundary_mutations::ConnectSurfaces> + From<crate::fenestration_mutations::CreateFenestration> + From<crate::CreateSurface> + From<crate::fenestration_geometry_mutations::ReplaceFenestrationVertices>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "surface", kind: "delete-surface", record: "DeletedSurface" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete surface {}", self.id.0), &format!("Oberfläche {} löschen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for DeleteSurface
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<DeleteSurface> + From<crate::boundary_mutations::ConnectSurfaces> + From<crate::fenestration_mutations::CreateFenestration> + From<crate::CreateSurface> + From<crate::fenestration_geometry_mutations::ReplaceFenestrationVertices>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }





    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
