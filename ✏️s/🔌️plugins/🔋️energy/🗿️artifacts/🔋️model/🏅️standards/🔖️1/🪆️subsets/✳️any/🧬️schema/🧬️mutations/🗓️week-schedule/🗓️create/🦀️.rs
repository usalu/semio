//! 🗓️ Energy model mutation — `CreateWeeklySchedule`: Defines one week as seven daily profiles, Sunday first (slot 0) through Saturday (slot 6) — the shape `ScheduleSet::weekly_value` indexes by day of week before it reads the hour.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🗓️ `create-weekly-schedule` payload. Defines one week as seven daily profiles, Sunday first (slot 0) through Saturday (slot 6) — the shape `ScheduleSet::weekly_value` indexes by day of week before it reads the hour.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-weekly-schedule")]
pub struct CreateWeeklySchedule {
    pub index: u32,
    pub id: crate::schedule::ScheduleId,
    pub daily_schedule_ids: Vec<crate::schedule::ScheduleId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_weekly_schedule<Op: From<CreateWeeklySchedule>>(index: u32, id: crate::schedule::ScheduleId, daily_schedule_ids: Vec<crate::schedule::ScheduleId>) -> Op {
    CreateWeeklySchedule { index, id, daily_schedule_ids }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateWeeklySchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateWeeklySchedule> + From<crate::DeleteWeeklySchedule>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create Weekly Schedule {} at index {}", self.id.0, self.index), &format!("Wochenzeitplan {} an Index {} erstellen", self.id.0, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateWeeklySchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateWeeklySchedule> + From<crate::DeleteWeeklySchedule>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "weekly-schedule", kind: "create-weekly-schedule", record: "CreatedWeeklySchedule" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
