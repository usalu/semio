//! 📗️ Energy model mutation — `InsertAnnualScheduleRule`: Places one date rule at a stated position in a year's ordered rule list. The order is load-bearing, not cosmetic — `ScheduleSet::annual_value` returns the FIRST rule whose date range contains the day — so this is `insert` with a FINAL-state index, not a set-like `add`.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📗️ `insert-annual-schedule-rule` payload. Places one date rule at a stated position in a year's ordered rule list. The order is load-bearing, not cosmetic — `ScheduleSet::annual_value` returns the FIRST rule whose date range contains the day — so this is `insert` with a FINAL-state index, not a set-like `add`.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "insert-annual-schedule-rule")]
pub struct InsertAnnualScheduleRule {
    pub id: crate::schedule::ScheduleId,
    pub index: u32,
    pub start_month: u8,
    pub start_day: u8,
    pub end_month: u8,
    pub end_day: u8,
    pub daily_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn insert_annual_schedule_rule<Op: From<InsertAnnualScheduleRule>>(id: crate::schedule::ScheduleId, index: u32, start_month: u8, start_day: u8, end_month: u8, end_day: u8, daily_schedule_id: crate::schedule::ScheduleId) -> Op {
    InsertAnnualScheduleRule { id, index, start_month, start_day, end_month, end_day, daily_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for InsertAnnualScheduleRule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<InsertAnnualScheduleRule> + From<crate::RemoveAnnualScheduleRule>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert rule at {} of annual schedule {}", self.index, self.id.0), &format!("Regel an Position {} in Jahreszeitplan {} einfügen", self.index, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for InsertAnnualScheduleRule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<InsertAnnualScheduleRule> + From<crate::RemoveAnnualScheduleRule>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "annual-schedule", kind: "insert-annual-schedule-rule", record: "InsertedAnnualScheduleRule" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
