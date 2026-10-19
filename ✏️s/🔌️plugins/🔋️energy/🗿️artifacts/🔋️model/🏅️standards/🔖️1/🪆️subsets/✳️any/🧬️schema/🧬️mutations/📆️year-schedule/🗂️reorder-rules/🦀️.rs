//! 🗂️ Energy model mutation — `ReorderAnnualScheduleRules`: Moves one date rule to another position in the year's rule list. This is the one `reorder` the schedule vocabulary carries, and it earns it: precedence between two rules whose date ranges overlap IS their list order.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🗂️ `reorder-annual-schedule-rules` payload. Moves one date rule to another position in the year's rule list. This is the one `reorder` the schedule vocabulary carries, and it earns it: precedence between two rules whose date ranges overlap IS their list order.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "reorder-annual-schedule-rules")]
pub struct ReorderAnnualScheduleRules {
    pub id: crate::schedule::ScheduleId,
    pub from: u32,
    pub to: u32,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn reorder_annual_schedule_rules<Op: From<ReorderAnnualScheduleRules>>(id: crate::schedule::ScheduleId, from: u32, to: u32) -> Op {
    ReorderAnnualScheduleRules { id, from, to }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for ReorderAnnualScheduleRules
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<ReorderAnnualScheduleRules>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "annual-schedule", kind: "reorder-annual-schedule-rules", record: "ReorderedAnnualScheduleRules" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Reorder annual schedule {} rule {} to {}", self.id.0, self.from, self.to), &format!("Jahreszeitplan {}: Regel {} an Position {} verschieben", self.id.0, self.from, self.to))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for ReorderAnnualScheduleRules
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<ReorderAnnualScheduleRules>,
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
