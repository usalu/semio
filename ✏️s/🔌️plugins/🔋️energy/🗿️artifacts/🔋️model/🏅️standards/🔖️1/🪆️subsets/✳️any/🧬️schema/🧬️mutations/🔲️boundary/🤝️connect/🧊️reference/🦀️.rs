//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::ConnectSurfaces;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ConnectSurfaces
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ConnectSurfaces> + From<crate::DisconnectSurfaces>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "surfaces", kind: "connect-surfaces", record: "ConnectedSurfaces" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect surfaces {} and {}", self.surface_a_id.0, self.surface_b_id.0), &format!("Oberflächen {} und {} verbinden", self.surface_a_id.0, self.surface_b_id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.surface_a_id.0.to_string(), self.surface_b_id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ConnectSurfaces
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ConnectSurfaces> + From<crate::DisconnectSurfaces>,
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
