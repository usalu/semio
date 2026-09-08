//! 📌️ Energy model mutation — `CreateSetpointManager`: Creates one loop setpoint manager. `SetpointManagerKind` is a tagged union with no `dsl::DslField`, so the variant travels as its wire name beside the four outdoor-air-reset limits, and every other variant has to carry them as zero.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 📌️ `create-setpoint-manager` payload. Creates one loop setpoint manager. `SetpointManagerKind` is a tagged union with no `dsl::DslField`, so the variant travels as its wire name beside the four outdoor-air-reset limits, and every other variant has to carry them as zero.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-setpoint-manager")]
pub struct CreateSetpointManager {
    pub id: crate::model::EntityId,
    pub name: String,
    pub kind: String,
    pub low_outdoor_c: f64,
    pub high_outdoor_c: f64,
    pub low_setpoint_c: f64,
    pub high_setpoint_c: f64,
    pub schedule_present: bool,
    pub schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_setpoint_manager(id: crate::model::EntityId, name: String, kind: String, low_outdoor_c: f64, high_outdoor_c: f64, low_setpoint_c: f64, high_setpoint_c: f64, schedule_present: bool, schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::CreateSetpointManager(CreateSetpointManager { id, name, kind, low_outdoor_c, high_outdoor_c, low_setpoint_c, high_setpoint_c, schedule_present, schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateSetpointManager {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "setpoint-manager", kind: "create-setpoint-manager", record: "CreatedSetpointManager" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create setpoint manager {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
