//! 🧇️ Energy model mutation — `ChangeZoneEquipmentHeatingCapacity`: Sets the heating output this equipment can deliver, in watts.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧇️ `change-zone-equipment-heating-capacity` payload. Sets the heating output this equipment can deliver, in watts.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-zone-equipment-heating-capacity")]
pub struct ChangeZoneEquipmentHeatingCapacity {
    pub id: crate::model::EntityId,
    pub new_heating_capacity_w: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_zone_equipment_heating_capacity(id: crate::model::EntityId, new_heating_capacity_w: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeZoneEquipmentHeatingCapacity(ChangeZoneEquipmentHeatingCapacity { id, new_heating_capacity_w })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeZoneEquipmentHeatingCapacity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "zone-equipment", kind: "change-zone-equipment-heating-capacity", record: "ChangedZoneEquipmentHeatingCapacity" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change zone equipment {} heating capacity to {:?}", self.id.0, self.new_heating_capacity_w)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
