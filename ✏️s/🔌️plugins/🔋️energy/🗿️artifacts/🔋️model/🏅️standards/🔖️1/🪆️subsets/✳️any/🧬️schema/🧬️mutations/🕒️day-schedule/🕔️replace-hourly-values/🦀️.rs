//! 🕔️ Energy model mutation — `ReplaceDailyScheduleHourlyValues`: Swaps a daily profile's whole twenty-four-value body. The hours are one shape, not twenty-four independent scalars, so they move together.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕔️ `replace-daily-schedule-hourly-values` payload. Swaps a daily profile's whole twenty-four-value body. The hours are one shape, not twenty-four independent scalars, so they move together.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-daily-schedule-hourly-values")]
pub struct ReplaceDailyScheduleHourlyValues {
    pub id: crate::schedule::ScheduleId,
    pub new_hourly_values: Vec<f64>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn replace_daily_schedule_hourly_values<Op: From<ReplaceDailyScheduleHourlyValues>>(id: crate::schedule::ScheduleId, new_hourly_values: Vec<f64>) -> Op {
    ReplaceDailyScheduleHourlyValues { id, new_hourly_values }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ReplaceDailyScheduleHourlyValues
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ReplaceDailyScheduleHourlyValues>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "daily-schedule", kind: "replace-daily-schedule-hourly-values", record: "ReplacedDailyScheduleHourlyValues" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace Daily Schedule Hourly Values of daily schedule {}", self.id.0), &format!("Stundenwerte von Tageszeitplan {} ersetzen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
