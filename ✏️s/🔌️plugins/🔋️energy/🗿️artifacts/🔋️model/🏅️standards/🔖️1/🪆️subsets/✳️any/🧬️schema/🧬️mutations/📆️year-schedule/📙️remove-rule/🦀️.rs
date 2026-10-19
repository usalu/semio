//! 📙️ Energy model mutation — `RemoveAnnualScheduleRule`: Takes one date rule out of a year's ordered rule list, addressed by its BASE-state index. Every later rule moves up one, and a day the removed rule used to answer falls through to the next matching rule or to the default profile.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📙️ `remove-annual-schedule-rule` payload. Takes one date rule out of a year's ordered rule list, addressed by its BASE-state index. Every later rule moves up one, and a day the removed rule used to answer falls through to the next matching rule or to the default profile.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-annual-schedule-rule")]
pub struct RemoveAnnualScheduleRule {
    pub id: crate::schedule::ScheduleId,
    pub index: u32,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn remove_annual_schedule_rule<Op: From<RemoveAnnualScheduleRule>>(id: crate::schedule::ScheduleId, index: u32) -> Op {
    RemoveAnnualScheduleRule { id, index }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for RemoveAnnualScheduleRule
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<RemoveAnnualScheduleRule> + From<crate::InsertAnnualScheduleRule>,
{


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove rule {} of annual schedule {}", self.index, self.id.0), &format!("Regel {} aus Jahreszeitplan {} entfernen", self.index, self.id.0))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for RemoveAnnualScheduleRule
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<RemoveAnnualScheduleRule> + From<crate::InsertAnnualScheduleRule>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }



    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "annual-schedule", kind: "remove-annual-schedule-rule", record: "RemovedAnnualScheduleRule" };


    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
