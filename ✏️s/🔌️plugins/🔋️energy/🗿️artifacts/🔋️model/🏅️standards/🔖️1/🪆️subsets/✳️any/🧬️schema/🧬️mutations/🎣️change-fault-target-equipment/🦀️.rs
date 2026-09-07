//! 🎣️ Energy model mutation — `ChangeFaultTargetEquipment`: Re-points one fault at another ideal loads system; a target the document does not define is refused.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎣️ `change-fault-target-equipment` payload. Re-points one fault at another ideal loads system; a target the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-fault-target-equipment")]
pub struct ChangeFaultTargetEquipment {
    pub id: crate::model::EntityId,
    pub new_target_equipment_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_fault_target_equipment(id: crate::model::EntityId, new_target_equipment_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeFaultTargetEquipment(ChangeFaultTargetEquipment { id, new_target_equipment_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeFaultTargetEquipment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fault", kind: "change-fault-target-equipment", record: "ChangedFaultTargetEquipment" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Fault Target Equipment of fault {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
