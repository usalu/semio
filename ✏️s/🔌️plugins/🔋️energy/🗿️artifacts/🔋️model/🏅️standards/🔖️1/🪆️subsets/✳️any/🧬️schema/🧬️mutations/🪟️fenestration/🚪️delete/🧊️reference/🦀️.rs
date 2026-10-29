//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::DeleteFenestration;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for DeleteFenestration
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<DeleteFenestration> + From<crate::CreateFenestration> + From<crate::fenestration_geometry_mutations::ReplaceFenestrationVertices>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "fenestration", kind: "delete-fenestration", record: "DeletedFenestration" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete fenestration {}", self.id.0), &format!("Fenster {} löschen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for DeleteFenestration
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<DeleteFenestration> + From<crate::CreateFenestration> + From<crate::fenestration_geometry_mutations::ReplaceFenestrationVertices>,
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
