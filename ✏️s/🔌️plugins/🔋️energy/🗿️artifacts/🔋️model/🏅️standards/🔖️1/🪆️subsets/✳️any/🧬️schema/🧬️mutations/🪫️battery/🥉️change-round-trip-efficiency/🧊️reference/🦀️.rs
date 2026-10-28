//! 🧊️ Whole-value reference semantics are separate from the authored retained payload and native producer.
use super::ChangeBatteryRoundTripEfficiency;
use crate::{snapshot::reference::EnergyModelSnapshot,diff::ReferenceEnergyModelDiff as EnergyModelDiff};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeBatteryRoundTripEfficiency
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeBatteryRoundTripEfficiency>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "battery", kind: "change-battery-round-trip-efficiency", record: "ChangedBatteryRoundTripEfficiency" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Battery Round Trip Efficiency of battery {}", self.id.0), &format!("Zyklenwirkungsgrad von Batterie {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeBatteryRoundTripEfficiency
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeBatteryRoundTripEfficiency>,
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
