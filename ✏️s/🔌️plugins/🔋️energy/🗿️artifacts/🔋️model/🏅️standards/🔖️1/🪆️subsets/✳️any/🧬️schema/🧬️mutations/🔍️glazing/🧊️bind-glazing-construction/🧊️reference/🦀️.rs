//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::BindFenestrationGlazingConstruction;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for BindFenestrationGlazingConstruction
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<BindFenestrationGlazingConstruction> + From<crate::ClearFenestrationGlazingConstruction>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "fenestration", kind: "bind-fenestration-glazing-construction", record: "BoundFenestrationGlazingConstruction" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Bind fenestration {} to glazing construction {}", self.id.0, self.construction_id.0), &format!("Fenster {} an Verglasungsaufbau {} binden", self.id.0, self.construction_id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for BindFenestrationGlazingConstruction
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<BindFenestrationGlazingConstruction> + From<crate::ClearFenestrationGlazingConstruction>,
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
