//! 📕️ Energy model mutation — `DeleteAnnualSchedule`: Removes one rule-based year. Refused while any consumer still resolves its id. Its inverse is a cascade: the create that re-defines the year, then one insert per date rule in order, then one add per holiday — because the rules and the holidays are collections of their own and the create does not carry them.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📕️ `delete-annual-schedule` payload. Removes one rule-based year. Refused while any consumer still resolves its id. Its inverse is a cascade: the create that re-defines the year, then one insert per date rule in order, then one add per holiday — because the rules and the holidays are collections of their own and the create does not carry them.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-annual-schedule")]
pub struct DeleteAnnualSchedule {
    pub id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_annual_schedule(id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteAnnualSchedule(DeleteAnnualSchedule { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteAnnualSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "annual-schedule", kind: "delete-annual-schedule", record: "DeletedAnnualSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete annual schedule {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
