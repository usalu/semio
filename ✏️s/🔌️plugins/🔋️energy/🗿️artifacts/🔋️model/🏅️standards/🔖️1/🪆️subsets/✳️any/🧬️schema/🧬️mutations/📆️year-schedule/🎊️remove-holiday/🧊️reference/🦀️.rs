//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::RemoveAnnualScheduleHoliday;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RemoveAnnualScheduleHoliday
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RemoveAnnualScheduleHoliday> + From<crate::AddAnnualScheduleHoliday>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "annual-schedule", kind: "remove-annual-schedule-holiday", record: "RemovedAnnualScheduleHoliday" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove holiday {}-{}-{} from annual schedule {}", self.year, self.month, self.day, self.id.0), &format!("Feiertag {}-{}-{} aus Jahreszeitplan {} entfernen", self.year, self.month, self.day, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for RemoveAnnualScheduleHoliday
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<RemoveAnnualScheduleHoliday> + From<crate::AddAnnualScheduleHoliday>,
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
