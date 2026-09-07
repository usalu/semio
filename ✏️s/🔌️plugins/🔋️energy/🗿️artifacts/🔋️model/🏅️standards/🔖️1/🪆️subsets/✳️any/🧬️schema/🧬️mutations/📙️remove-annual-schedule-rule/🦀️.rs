//! 📙️ Energy model mutation — `RemoveAnnualScheduleRule`: Takes one date rule out of a year's ordered rule list, addressed by its BASE-state index. Every later rule moves up one, and a day the removed rule used to answer falls through to the next matching rule or to the default profile.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📙️ `remove-annual-schedule-rule` payload. Takes one date rule out of a year's ordered rule list, addressed by its BASE-state index. Every later rule moves up one, and a day the removed rule used to answer falls through to the next matching rule or to the default profile.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-annual-schedule-rule")]
pub struct RemoveAnnualScheduleRule {
    pub id: crate::model::ScheduleId,
    pub index: u32,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_annual_schedule_rule(id: crate::model::ScheduleId, index: u32) -> EnergyModelMutation {
    EnergyModelMutation::RemoveAnnualScheduleRule(RemoveAnnualScheduleRule { id, index })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RemoveAnnualScheduleRule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "annual-schedule", kind: "remove-annual-schedule-rule", record: "RemovedAnnualScheduleRule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Remove rule {} of annual schedule {}", self.index, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
