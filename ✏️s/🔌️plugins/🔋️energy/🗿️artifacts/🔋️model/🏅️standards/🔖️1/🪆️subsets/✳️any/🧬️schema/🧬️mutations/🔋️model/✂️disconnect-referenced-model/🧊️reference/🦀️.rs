//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::DisconnectReferencedModel;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for DisconnectReferencedModel
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<DisconnectReferencedModel> + From<crate::ConnectReferencedModel>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "referenced-model", kind: "disconnect-referenced-model", record: "DisconnectedReferencedModel" };





    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Disconnect the referenced model", "Referenziertes Modell trennen")
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for DisconnectReferencedModel
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<DisconnectReferencedModel> + From<crate::ConnectReferencedModel>,
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
