//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::DeleteConstruction;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for DeleteConstruction
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<DeleteConstruction> + From<crate::CreateConstruction>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "construction", kind: "delete-construction", record: "DeletedConstruction" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete Construction {}", self.id.0), &format!("Konstruktion {} löschen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for DeleteConstruction
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<DeleteConstruction> + From<crate::CreateConstruction>,
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
