//! 🧊️ Whole-value reference semantics are separate from the authored retained payload and native producer.
use super::ChangeOutdoorAirSystemEconomizerEnabled;
use crate::{snapshot::reference::EnergyModelSnapshot,diff::ReferenceEnergyModelDiff as EnergyModelDiff};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeOutdoorAirSystemEconomizerEnabled
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeOutdoorAirSystemEconomizerEnabled>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "outdoor-air-system", kind: "change-outdoor-air-system-economizer-enabled", record: "ChangedOutdoorAirSystemEconomizerEnabled" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change outdoor air system {} economizer setting to {:?}", self.id.0, self.new_economizer_enabled), &format!("Economizer-Einstellung von Außenluftsystem {} auf {:?} ändern", self.id.0, self.new_economizer_enabled))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeOutdoorAirSystemEconomizerEnabled
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeOutdoorAirSystemEconomizerEnabled>,
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
