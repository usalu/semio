//! ⚠️ Energy model mutation — `CreateFault`: Adds one equipment fault. `targetEquipmentId` is checked against `ideal_loads` because that is the collection the kernel actually matches it against (`SystemSubstepStage::Fault` compares `fault.target_equipment_id == ideal.id`); the field's own type carries no discriminator, so this is the only referent the engine gives it.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚠️ `create-fault` payload. Adds one equipment fault. `targetEquipmentId` is checked against `ideal_loads` because that is the collection the kernel actually matches it against (`SystemSubstepStage::Fault` compares `fault.target_equipment_id == ideal.id`); the field's own type carries no discriminator, so this is the only referent the engine gives it.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-fault")]
pub struct CreateFault {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub target_equipment_id: crate::model::EntityId,
    pub fault_type: crate::model::FaultType,
    pub severity: f64,
    pub start_schedule_id: crate::model::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_fault(index: u32, id: crate::model::EntityId, target_equipment_id: crate::model::EntityId, fault_type: crate::model::FaultType, severity: f64, start_schedule_id: crate::model::ScheduleId) -> EnergyModelMutation {
    EnergyModelMutation::CreateFault(CreateFault { index, id, target_equipment_id, fault_type, severity, start_schedule_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateFault {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "fault", kind: "create-fault", record: "CreatedFault" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create Fault {} at index {}", self.id.0, self.index)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
