//! 🧊️ Whole-value reference semantics are separate from the authored retained payload and native producer.
use super::ChangeEquipmentGainRadiantFraction;
use crate::{snapshot::reference::EnergyModelSnapshot,diff::ReferenceEnergyModelDiff as EnergyModelDiff};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeEquipmentGainRadiantFraction
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeEquipmentGainRadiantFraction>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "equipment-gain", kind: "change-equipment-gain-radiant-fraction", record: "ChangedEquipmentGainRadiantFraction" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Equipment Gain Radiant Fraction of equipment gain {}", self.id.0), &format!("Strahlungsanteil von Gerätewärmegewinn {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeEquipmentGainRadiantFraction
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeEquipmentGainRadiantFraction>,
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
