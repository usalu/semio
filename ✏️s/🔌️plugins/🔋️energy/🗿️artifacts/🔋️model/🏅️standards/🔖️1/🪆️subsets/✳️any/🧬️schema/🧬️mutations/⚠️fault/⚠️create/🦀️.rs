//! ⚠️ Energy model mutation — `CreateFault`: Adds one equipment fault. `targetEquipmentId` is checked against `ideal_loads` because that is the collection the kernel actually matches it against (`SystemSubstepStage::Fault` compares `fault.target_equipment_id == ideal.id`); the field's own type carries no discriminator, so this is the only referent the engine gives it.

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// ⚠️ `create-fault` payload. Adds one equipment fault. `targetEquipmentId` is checked against `ideal_loads` because that is the collection the kernel actually matches it against (`SystemSubstepStage::Fault` compares `fault.target_equipment_id == ideal.id`); the field's own type carries no discriminator, so this is the only referent the engine gives it.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-fault")]
pub struct CreateFault {
    pub index: u32,
    pub id: crate::model::EntityId,
    pub target_equipment_id: crate::model::EntityId,
    pub fault_type: crate::model::FaultType,
    pub severity: f64,
    pub start_schedule_id: crate::schedule::ScheduleId,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn create_fault<Op: From<CreateFault>>(index: u32, id: crate::model::EntityId, target_equipment_id: crate::model::EntityId, fault_type: crate::model::FaultType, severity: f64, start_schedule_id: crate::schedule::ScheduleId) -> Op {
    CreateFault { index, id, target_equipment_id, fault_type, severity, start_schedule_id }.into()
}

impl<Op> protocol::MutationKind<EnergyModelSnapshot, Op> for CreateFault
where
    Op: protocol::Mutation<EnergyModelSnapshot, Diff = EnergyModelDiff> + From<CreateFault> + From<crate::DeleteFault>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "fault", kind: "create-fault", record: "CreatedFault" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create Fault {} at index {}", self.id.0, self.index), &format!("Fehler {} an Index {} erstellen", self.id.0, self.index))
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<EnergyModelSnapshot, Op> for CreateFault
where
    Op: protocol::ColdMutation<EnergyModelSnapshot, ReferenceDiff = EnergyModelDiff> + From<CreateFault> + From<crate::DeleteFault>,
{
    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }





    fn inverse(&self, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
