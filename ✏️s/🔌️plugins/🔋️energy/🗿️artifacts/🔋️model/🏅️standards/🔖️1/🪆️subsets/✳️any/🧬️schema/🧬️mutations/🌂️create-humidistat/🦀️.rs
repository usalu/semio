//! 🌂️ Energy model mutation — `CreateHumidistat`: Creates the humidity control of one zone, against two setpoint schedules the model already defines.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌂️ `create-humidistat` payload. Creates the humidity control of one zone, against two setpoint schedules the model already defines.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-humidistat")]
pub struct CreateHumidistat {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub humidifying_setpoint_schedule_id: crate::model::ScheduleId,
    pub dehumidifying_setpoint_schedule_id: crate::model::ScheduleId,
    pub humidifying_throttle_range: f64,
    pub dehumidifying_throttle_range: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_humidistat(id: crate::model::EntityId, zone_id: crate::model::EntityId, humidifying_setpoint_schedule_id: crate::model::ScheduleId, dehumidifying_setpoint_schedule_id: crate::model::ScheduleId, humidifying_throttle_range: f64, dehumidifying_throttle_range: f64) -> EnergyModelMutation {
    EnergyModelMutation::CreateHumidistat(CreateHumidistat { id, zone_id, humidifying_setpoint_schedule_id, dehumidifying_setpoint_schedule_id, humidifying_throttle_range, dehumidifying_throttle_range })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateHumidistat {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "humidistat", kind: "create-humidistat", record: "CreatedHumidistat" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create humidistat {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
