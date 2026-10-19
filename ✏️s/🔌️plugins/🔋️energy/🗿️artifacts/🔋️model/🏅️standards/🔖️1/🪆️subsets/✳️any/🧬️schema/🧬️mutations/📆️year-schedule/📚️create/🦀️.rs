//! 📚️ Energy model mutation — `CreateAnnualSchedule`: Defines one rule-based year. It starts with no date rules and no holidays — those are ordered and set-like collections of their own, added by `insert-annual-schedule-rule` and `add-annual-schedule-holiday` — so a create states only the two fallbacks every lookup ends at.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📚️ `create-annual-schedule` payload. Defines one rule-based year. It starts with no date rules and no holidays — those are ordered and set-like collections of their own, added by `insert-annual-schedule-rule` and `add-annual-schedule-holiday` — so a create states only the two fallbacks every lookup ends at.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-annual-schedule")]
pub struct CreateAnnualSchedule {
    pub index: u32,
    pub id: crate::schedule::ScheduleId,
    pub default_daily_schedule_id: crate::schedule::ScheduleId,
    pub holiday_daily_schedule_id: Option<crate::schedule::ScheduleId>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_annual_schedule<Op: From<CreateAnnualSchedule>>(index: u32, id: crate::schedule::ScheduleId, default_daily_schedule_id: crate::schedule::ScheduleId, holiday_daily_schedule_id: Option<crate::schedule::ScheduleId>) -> Op {
    CreateAnnualSchedule { index, id, default_daily_schedule_id, holiday_daily_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateAnnualSchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateAnnualSchedule> + From<crate::DeleteAnnualSchedule>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create Annual Schedule {} at index {}", self.id.0, self.index), &format!("Jahreszeitplan {} an Index {} erstellen", self.id.0, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateAnnualSchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateAnnualSchedule> + From<crate::DeleteAnnualSchedule>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "annual-schedule", kind: "create-annual-schedule", record: "CreatedAnnualSchedule" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
