//! 🔧️ Energy model mutation — `ChangeZoneEquipmentType`: Swaps the catalog type the assignment names. The eight-way enum is one scalar, not eight fields, so it changes as a whole.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔧️ `change-zone-equipment-type` payload. Swaps the catalog type the assignment names. The eight-way enum is one scalar, not eight fields, so it changes as a whole.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-equipment-type")]
pub struct ChangeZoneEquipmentType {
    pub id: crate::model::EntityId,
    pub new_equipment_type: crate::model::ZoneEquipmentType,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_zone_equipment_type(id: crate::model::EntityId, new_equipment_type: crate::model::ZoneEquipmentType) -> EnergyModelMutation {
    EnergyModelMutation::ChangeZoneEquipmentType(ChangeZoneEquipmentType { id, new_equipment_type })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeZoneEquipmentType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone-equipment", kind: "change-zone-equipment-type", record: "ChangedZoneEquipmentType" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change zone equipment {} equipment type to {:?}", self.id.0, self.new_equipment_type)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
