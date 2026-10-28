//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::RemoveOutputVariable;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RemoveOutputVariable
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RemoveOutputVariable> + From<crate::AddOutputVariable>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "output-variable", kind: "remove-output-variable", record: "RemovedOutputVariable" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove output variable \"{}\" for \"{}\"", self.name, self.key), &format!("Ausgabevariable \"{}\" für \"{}\" entfernen", self.name, self.key))
    }

    fn target(&self) -> Vec<String> {
        vec![self.name.clone(), self.key.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for RemoveOutputVariable
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<RemoveOutputVariable> + From<crate::AddOutputVariable>,
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
