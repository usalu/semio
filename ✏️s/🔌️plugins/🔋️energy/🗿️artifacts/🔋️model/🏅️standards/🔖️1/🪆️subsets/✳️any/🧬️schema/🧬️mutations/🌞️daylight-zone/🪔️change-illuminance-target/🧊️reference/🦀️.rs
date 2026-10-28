//! 🧊️ Whole-value reference semantics are separate from the authored retained payload and native producer.
use super::ChangeDaylightZoneIlluminanceTarget;
use crate::{snapshot::reference::EnergyModelSnapshot,diff::ReferenceEnergyModelDiff as EnergyModelDiff};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeDaylightZoneIlluminanceTarget
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeDaylightZoneIlluminanceTarget>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "daylight-zone", kind: "change-daylight-zone-illuminance-target", record: "ChangedDaylightZoneIlluminanceTarget" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change daylight zone {} illuminance target to {:?}", self.id.0, self.new_illuminance_target_lux), &format!("Soll-Beleuchtungsstärke von Tageslichtzone {} auf {:?} ändern", self.id.0, self.new_illuminance_target_lux))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeDaylightZoneIlluminanceTarget
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeDaylightZoneIlluminanceTarget>,
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
