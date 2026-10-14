//! 🎌️ Energy model mutation — `ChangeAnnualScheduleDefaultDailySchedule`: Re-points the profile a year falls back to on every day no rule matches.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎌️ `change-annual-schedule-default-daily-schedule` payload. Re-points the profile a year falls back to on every day no rule matches.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-annual-schedule-default-daily-schedule")]
pub struct ChangeAnnualScheduleDefaultDailySchedule {
    pub id: crate::schedule::ScheduleId,
    pub new_default_daily_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_annual_schedule_default_daily_schedule<Op: From<ChangeAnnualScheduleDefaultDailySchedule>>(id: crate::schedule::ScheduleId, new_default_daily_schedule_id: crate::schedule::ScheduleId) -> Op {
    ChangeAnnualScheduleDefaultDailySchedule { id, new_default_daily_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeAnnualScheduleDefaultDailySchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeAnnualScheduleDefaultDailySchedule>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "annual-schedule", kind: "change-annual-schedule-default-daily-schedule", record: "ChangedAnnualScheduleDefaultDailySchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Annual Schedule Default Daily Schedule of annual schedule {}", self.id.0), &format!("Standard-Tageszeitplan von Jahreszeitplan {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
