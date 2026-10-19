//! 🕙️ Energy model mutation — `ChangeTimeSeriesScheduleTimestep`: Sets how many seconds one sample of a measured series covers — what turns its index into a clock.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕙️ `change-time-series-schedule-timestep` payload. Sets how many seconds one sample of a measured series covers — what turns its index into a clock.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-time-series-schedule-timestep")]
pub struct ChangeTimeSeriesScheduleTimestep {
    pub id: crate::schedule::ScheduleId,
    pub new_timestep_seconds: u32,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_time_series_schedule_timestep<Op: From<ChangeTimeSeriesScheduleTimestep>>(id: crate::schedule::ScheduleId, new_timestep_seconds: u32) -> Op {
    ChangeTimeSeriesScheduleTimestep { id, new_timestep_seconds }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ChangeTimeSeriesScheduleTimestep
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ChangeTimeSeriesScheduleTimestep>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "time-series-schedule", kind: "change-time-series-schedule-timestep", record: "ChangedTimeSeriesScheduleTimestep" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change Time Series Schedule Timestep of time series schedule {}", self.id.0), &format!("Zeitschritt von Zeitreihenzeitplan {} ändern", self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ChangeTimeSeriesScheduleTimestep
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ChangeTimeSeriesScheduleTimestep>,
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
