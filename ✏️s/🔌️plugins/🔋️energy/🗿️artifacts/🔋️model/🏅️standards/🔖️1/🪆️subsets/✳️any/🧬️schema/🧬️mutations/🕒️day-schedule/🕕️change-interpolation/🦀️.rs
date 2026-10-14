//! 🕕️ Energy model mutation — `ChangeDailyScheduleInterpolation`: Sets whether a daily profile steps between its hourly values or ramps across them.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕕️ `change-daily-schedule-interpolation` payload. Sets whether a daily profile steps between its hourly values or ramps across them.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-daily-schedule-interpolation")]
pub struct ChangeDailyScheduleInterpolation {
    pub id: crate::schedule::ScheduleId,
    pub new_interpolation: crate::schedule::ScheduleInterpolation,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_daily_schedule_interpolation<Op: From<ChangeDailyScheduleInterpolation>>(id: crate::schedule::ScheduleId, new_interpolation: crate::schedule::ScheduleInterpolation) -> Op {
    ChangeDailyScheduleInterpolation { id, new_interpolation }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeDailyScheduleInterpolation
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeDailyScheduleInterpolation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "daily-schedule", kind: "change-daily-schedule-interpolation", record: "ChangedDailyScheduleInterpolation" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Daily Schedule Interpolation of daily schedule {}", self.id.0), &format!("Interpolation von Tageszeitplan {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
