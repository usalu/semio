//! 🕞️ Energy model mutation — `CreateDailySchedule`: Defines one twenty-four-hour profile. The optional lower and upper bound are one facet — both together or neither — and the values are clamped to them on every lookup, which is why a half-stated pair is refused rather than half-applied.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕞️ `create-daily-schedule` payload. Defines one twenty-four-hour profile. The optional lower and upper bound are one facet — both together or neither — and the values are clamped to them on every lookup, which is why a half-stated pair is refused rather than half-applied.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-daily-schedule")]
pub struct CreateDailySchedule {
    pub index: u32,
    pub id: crate::schedule::ScheduleId,
    pub hourly_values: Vec<f64>,
    pub interpolation: crate::schedule::ScheduleInterpolation,
    pub limits_min: Option<f64>,
    pub limits_max: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_daily_schedule<Op: From<CreateDailySchedule>>(index: u32, id: crate::schedule::ScheduleId, hourly_values: Vec<f64>, interpolation: crate::schedule::ScheduleInterpolation, limits_min: Option<f64>, limits_max: Option<f64>) -> Op {
    CreateDailySchedule { index, id, hourly_values, interpolation, limits_min, limits_max }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateDailySchedule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateDailySchedule> + From<crate::DeleteDailySchedule>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create Daily Schedule {} at index {}", self.id.0, self.index), &format!("Tageszeitplan {} an Index {} erstellen", self.id.0, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateDailySchedule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateDailySchedule> + From<crate::DeleteDailySchedule>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "daily-schedule", kind: "create-daily-schedule", record: "CreatedDailySchedule" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
