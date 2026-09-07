//! 🕑️ Energy model mutation — `ChangeRefrigerationSystemDefrostSchedule`: Re-points one refrigeration system's schedule reference; a schedule the document does not define is refused.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕑️ `change-refrigeration-system-defrost-schedule` payload. Re-points one refrigeration system's schedule reference; a schedule the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-refrigeration-system-defrost-schedule")]
pub struct ChangeRefrigerationSystemDefrostSchedule {
    pub id: crate::model::EntityId,
    pub new_defrost_schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_refrigeration_system_defrost_schedule(id: crate::model::EntityId, new_defrost_schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeRefrigerationSystemDefrostSchedule(ChangeRefrigerationSystemDefrostSchedule { id, new_defrost_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeRefrigerationSystemDefrostSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "refrigeration-system", kind: "change-refrigeration-system-defrost-schedule", record: "ChangedRefrigerationSystemDefrostSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Refrigeration System Defrost Schedule of refrigeration system {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
