//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::RemovePlantLoopEquipment;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RemovePlantLoopEquipment
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RemovePlantLoopEquipment> + From<crate::AddPlantLoopEquipment>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "plant-loop", kind: "remove-plant-loop-equipment", record: "RemovedPlantLoopEquipment" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove plant equipment {} from plant loop {}", self.equipment_id.0, self.id.0), &format!("Anlagenkomponente {} aus Anlagenkreislauf {} entfernen", self.equipment_id.0, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string(), self.equipment_id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for RemovePlantLoopEquipment
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<RemovePlantLoopEquipment> + From<crate::AddPlantLoopEquipment>,
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
