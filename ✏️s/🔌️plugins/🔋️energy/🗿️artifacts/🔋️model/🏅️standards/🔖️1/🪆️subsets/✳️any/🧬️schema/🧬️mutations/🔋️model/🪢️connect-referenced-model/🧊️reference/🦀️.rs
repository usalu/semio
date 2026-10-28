//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::ConnectReferencedModel;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ConnectReferencedModel
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ConnectReferencedModel> + From<crate::DisconnectReferencedModel>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "referenced-model", kind: "connect-referenced-model", record: "ConnectedReferencedModel" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect referenced model {}", self.link.target.artifact_id), &format!("Referenziertes Modell {} verbinden", self.link.target.artifact_id))
    }

    fn target(&self) -> Vec<String> {
        vec![self.link.target.artifact_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ConnectReferencedModel
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ConnectReferencedModel> + From<crate::DisconnectReferencedModel>,
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
