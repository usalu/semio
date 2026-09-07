//! 🖨️ Energy model mutation — `ChangeEquipmentGainZone`: Re-points one equipment gain's zone reference; a zone the document does not define is refused.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🖨️ `change-equipment-gain-zone` payload. Re-points one equipment gain's zone reference; a zone the document does not define is refused.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-equipment-gain-zone")]
pub struct ChangeEquipmentGainZone {
    pub id: crate::model::EntityId,
    pub new_zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_equipment_gain_zone(id: crate::model::EntityId, new_zone_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeEquipmentGainZone(ChangeEquipmentGainZone { id, new_zone_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeEquipmentGainZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "equipment-gain", kind: "change-equipment-gain-zone", record: "ChangedEquipmentGainZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Equipment Gain Zone of equipment gain {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
