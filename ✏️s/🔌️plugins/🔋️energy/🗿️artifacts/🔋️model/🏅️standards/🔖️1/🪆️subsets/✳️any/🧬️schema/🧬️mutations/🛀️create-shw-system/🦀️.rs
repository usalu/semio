//! 🛀️ Energy model mutation — `CreateShwSystem`: Adds one service-hot-water system: a storage tank, the heater that keeps it at setpoint, and the draw schedule the load profile is read from.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛀️ `create-shw-system` payload. Adds one service-hot-water system: a storage tank, the heater that keeps it at setpoint, and the draw schedule the load profile is read from.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-shw-system")]
pub struct CreateShwSystem {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub heater_capacity_w: f64,
    pub storage_volume_m3: f64,
    pub setpoint_c: f64,
    pub schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_shw_system(index: u32, id: crate::model::EntityId, heater_capacity_w: f64, storage_volume_m3: f64, setpoint_c: f64, schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::CreateShwSystem(CreateShwSystem { index, id, heater_capacity_w, storage_volume_m3, setpoint_c, schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateShwSystem {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "service-hot-water-system", kind: "create-shw-system", record: "CreatedShwSystem" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Shw System {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
