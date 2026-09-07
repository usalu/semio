//! 🛠️ Energy model mutation — `CreateZoneEquipment`: Adds one piece of zone equipment to a zone's own equipment list. `priority` is its rank in that list, so it starts at one.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛠️ `create-zone-equipment` payload. Adds one piece of zone equipment to a zone's own equipment list. `priority` is its rank in that list, so it starts at one.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-zone-equipment")]
pub struct CreateZoneEquipment {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
    pub equipment_type: crate::model::ZoneEquipmentType,
    pub priority: u8,
    pub heating_capacity_w: f64,
    pub cooling_capacity_w: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_zone_equipment(id: crate::model::EntityId, zone_id: crate::model::EntityId, equipment_type: crate::model::ZoneEquipmentType, priority: u8, heating_capacity_w: f64, cooling_capacity_w: f64) -> EnergyModelMutation {
    EnergyModelMutation::CreateZoneEquipment(CreateZoneEquipment { id, zone_id, equipment_type, priority, heating_capacity_w, cooling_capacity_w })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for CreateZoneEquipment {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "zone-equipment", kind: "create-zone-equipment", record: "CreatedZoneEquipment" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create zone equipment {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
