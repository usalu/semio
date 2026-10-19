//! 🕖️ Energy model mutation — `DeleteWeeklySchedule`: Removes one weekly schedule. Refused while any consumer still resolves its id.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕖️ `delete-weekly-schedule` payload. Removes one weekly schedule. Refused while any consumer still resolves its id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-weekly-schedule")]
pub struct DeleteWeeklySchedule {
    pub id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn delete_weekly_schedule<Op: From<DeleteWeeklySchedule>>(id: crate::schedule::ScheduleId) -> Op {
    DeleteWeeklySchedule { id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for DeleteWeeklySchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<DeleteWeeklySchedule> + From<crate::CreateWeeklySchedule>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "weekly-schedule", kind: "delete-weekly-schedule", record: "DeletedWeeklySchedule" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete Weekly Schedule {}", self.id.0), &format!("Wochenzeitplan {} löschen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for DeleteWeeklySchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<DeleteWeeklySchedule> + From<crate::CreateWeeklySchedule>,
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
//#endregion 🔖️Mutation
