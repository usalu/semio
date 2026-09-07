//! 🗂️ Energy model mutation — `ReorderAnnualScheduleRules`: Moves one date rule to another position in the year's rule list. This is the one `reorder` the schedule vocabulary carries, and it earns it: precedence between two rules whose date ranges overlap IS their list order.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🗂️ `reorder-annual-schedule-rules` payload. Moves one date rule to another position in the year's rule list. This is the one `reorder` the schedule vocabulary carries, and it earns it: precedence between two rules whose date ranges overlap IS their list order.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "reorder-annual-schedule-rules")]
pub struct ReorderAnnualScheduleRules {
    pub id: crate::model::ScheduleId,
    pub from: u32,
    pub to: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn reorder_annual_schedule_rules(id: crate::model::ScheduleId, from: u32, to: u32) -> EnergyModelMutation {
    EnergyModelMutation::ReorderAnnualScheduleRules(ReorderAnnualScheduleRules { id, from, to })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ReorderAnnualScheduleRules {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "annual-schedule", kind: "reorder-annual-schedule-rules", record: "ReorderedAnnualScheduleRules" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Reorder annual schedule {} rule {} to {}", self.id.0, self.from, self.to)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
