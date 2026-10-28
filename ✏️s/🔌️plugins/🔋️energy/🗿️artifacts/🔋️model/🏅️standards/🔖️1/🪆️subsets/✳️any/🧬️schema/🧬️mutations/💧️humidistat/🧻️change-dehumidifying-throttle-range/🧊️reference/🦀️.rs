//! 🧊️ Whole-value reference semantics are separate from the authored retained payload and native producer.
use super::ChangeHumidistatDehumidifyingThrottleRange;
use crate::{snapshot::reference::EnergyModelSnapshot,diff::ReferenceEnergyModelDiff as EnergyModelDiff};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeHumidistatDehumidifyingThrottleRange
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeHumidistatDehumidifyingThrottleRange>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "humidistat", kind: "change-humidistat-dehumidifying-throttle-range", record: "ChangedHumidistatDehumidifyingThrottleRange" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change humidistat {} dehumidifying throttle range to {:?}", self.id.0, self.new_dehumidifying_throttle_range), &format!("Entfeuchtungs-Proportionalbereich von Feuchteregler {} auf {:?} ändern", self.id.0, self.new_dehumidifying_throttle_range))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeHumidistatDehumidifyingThrottleRange
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeHumidistatDehumidifyingThrottleRange>,
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
