//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::AddAnnualScheduleHoliday;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for AddAnnualScheduleHoliday
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<AddAnnualScheduleHoliday> + From<crate::RemoveAnnualScheduleHoliday>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "annual-schedule", kind: "add-annual-schedule-holiday", record: "AddedAnnualScheduleHoliday" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Add holiday {}-{}-{} to annual schedule {}", self.year, self.month, self.day, self.id.0), &format!("Feiertag {}-{}-{} zu Jahreszeitplan {} hinzufügen", self.year, self.month, self.day, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for AddAnnualScheduleHoliday
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<AddAnnualScheduleHoliday> + From<crate::RemoveAnnualScheduleHoliday>,
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
