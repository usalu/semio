//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::DisconnectSurfaces;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for DisconnectSurfaces
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<DisconnectSurfaces> + From<crate::ConnectSurfaces>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "surfaces", kind: "disconnect-surfaces", record: "DisconnectedSurfaces" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Disconnect surfaces {} and {}", self.surface_a_id.0, self.surface_b_id.0), &format!("Oberflächen {} und {} trennen", self.surface_a_id.0, self.surface_b_id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.surface_a_id.0.to_string(), self.surface_b_id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for DisconnectSurfaces
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<DisconnectSurfaces> + From<crate::ConnectSurfaces>,
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
