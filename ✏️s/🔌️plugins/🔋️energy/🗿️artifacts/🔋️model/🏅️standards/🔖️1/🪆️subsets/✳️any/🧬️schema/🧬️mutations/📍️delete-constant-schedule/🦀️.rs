//! 📍️ Energy model mutation — `DeleteConstantSchedule`: Removes one constant schedule. Refused while any gain, thermostat, system or other schedule still resolves its id — the document would otherwise carry a reference `Model::validate` reports as severe.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📍️ `delete-constant-schedule` payload. Removes one constant schedule. Refused while any gain, thermostat, system or other schedule still resolves its id — the document would otherwise carry a reference `Model::validate` reports as severe.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-constant-schedule")]
pub struct DeleteConstantSchedule {
    pub id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_constant_schedule(id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteConstantSchedule(DeleteConstantSchedule { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteConstantSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "constant-schedule", kind: "delete-constant-schedule", record: "DeletedConstantSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete Constant Schedule {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
