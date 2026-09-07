//! 🎗️ Energy model mutation — `ChangeZoneEquipmentPriority`: Sets the rank this equipment holds in its zone's own equipment list.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🎗️ `change-zone-equipment-priority` payload. Sets the rank this equipment holds in its zone's own equipment list.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-equipment-priority")]
pub struct ChangeZoneEquipmentPriority {
    pub id: crate::model::EntityId,
    pub new_priority: u8,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_zone_equipment_priority(id: crate::model::EntityId, new_priority: u8) -> EnergyModelMutation {
    EnergyModelMutation::ChangeZoneEquipmentPriority(ChangeZoneEquipmentPriority { id, new_priority })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeZoneEquipmentPriority {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone-equipment", kind: "change-zone-equipment-priority", record: "ChangedZoneEquipmentPriority" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change zone equipment {} priority to {:?}", self.id.0, self.new_priority)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
