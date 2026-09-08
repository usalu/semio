//! 🕜️ Energy model mutation — `CreateConstantSchedule`: Defines one schedule that holds the same value at every timestep — the shape a fixed setpoint, a fixed fraction or an always-on availability takes.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🕜️ `create-constant-schedule` payload. Defines one schedule that holds the same value at every timestep — the shape a fixed setpoint, a fixed fraction or an always-on availability takes.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-constant-schedule")]
pub struct CreateConstantSchedule {
    pub index: u32,
    pub id: crate::model::ScheduleId,
    pub value: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_constant_schedule(index: u32, id: crate::model::ScheduleId, value: f64) -> EnergyModelMutation {
    EnergyModelMutation::CreateConstantSchedule(CreateConstantSchedule { index, id, value })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateConstantSchedule {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "constant-schedule", kind: "create-constant-schedule", record: "CreatedConstantSchedule" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Constant Schedule {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
