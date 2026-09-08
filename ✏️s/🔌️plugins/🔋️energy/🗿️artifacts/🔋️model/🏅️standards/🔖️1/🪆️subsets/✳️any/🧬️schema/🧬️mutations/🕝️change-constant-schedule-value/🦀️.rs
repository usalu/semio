//! 🕝️ Energy model mutation — `ChangeConstantScheduleValue`: Sets the one value a constant schedule returns at every timestep.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕝️ `change-constant-schedule-value` payload. Sets the one value a constant schedule returns at every timestep.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-constant-schedule-value")]
pub struct ChangeConstantScheduleValue {
    pub id: crate::model::ScheduleId,
    pub new_value: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_constant_schedule_value(id: crate::model::ScheduleId, new_value: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeConstantScheduleValue(ChangeConstantScheduleValue { id, new_value })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeConstantScheduleValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "constant-schedule", kind: "change-constant-schedule-value", record: "ChangedConstantScheduleValue" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Constant Schedule Value of constant schedule {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
