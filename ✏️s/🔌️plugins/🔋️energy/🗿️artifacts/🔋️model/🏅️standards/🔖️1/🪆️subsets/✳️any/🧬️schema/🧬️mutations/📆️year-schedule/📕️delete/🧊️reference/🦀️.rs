//! 🧊️ Explicit reference behavior retains its declared concrete cold capabilities.
use super::DeleteAnnualSchedule;
use crate::{diff::ReferenceEnergyModelDiff as EnergyModelDiff,snapshot::reference::EnergyModelSnapshot};
impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for DeleteAnnualSchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<DeleteAnnualSchedule> + From<crate::year_schedule_holidays_mutations::AddAnnualScheduleHoliday> + From<crate::CreateAnnualSchedule> + From<crate::InsertAnnualScheduleRule>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "annual-schedule", kind: "delete-annual-schedule", record: "DeletedAnnualSchedule" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete annual schedule {}", self.id.0), &format!("Jahreszeitplan {} löschen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for DeleteAnnualSchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<DeleteAnnualSchedule> + From<crate::year_schedule_holidays_mutations::AddAnnualScheduleHoliday> + From<crate::CreateAnnualSchedule> + From<crate::InsertAnnualScheduleRule>,
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
