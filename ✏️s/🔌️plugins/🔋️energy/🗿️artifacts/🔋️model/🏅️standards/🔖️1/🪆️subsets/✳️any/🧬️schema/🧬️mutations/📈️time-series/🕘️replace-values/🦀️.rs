//! 🕘️ Energy model mutation — `ReplaceTimeSeriesScheduleValues`: Swaps a measured series' whole body. The samples are one measurement, not independent scalars, so they move together.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕘️ `replace-time-series-schedule-values` payload. Swaps a measured series' whole body. The samples are one measurement, not independent scalars, so they move together.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-time-series-schedule-values")]
pub struct ReplaceTimeSeriesScheduleValues {
    pub id: crate::schedule::ScheduleId,
    pub new_values: Vec<f64>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn replace_time_series_schedule_values<Op: From<ReplaceTimeSeriesScheduleValues>>(id: crate::schedule::ScheduleId, new_values: Vec<f64>) -> Op {
    ReplaceTimeSeriesScheduleValues { id, new_values }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ReplaceTimeSeriesScheduleValues
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ReplaceTimeSeriesScheduleValues>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "time-series-schedule", kind: "replace-time-series-schedule-values", record: "ReplacedTimeSeriesScheduleValues" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Replace Time Series Schedule Values of time series schedule {}", self.id.0), &format!("Werte von Zeitreihenzeitplan {} ersetzen", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
