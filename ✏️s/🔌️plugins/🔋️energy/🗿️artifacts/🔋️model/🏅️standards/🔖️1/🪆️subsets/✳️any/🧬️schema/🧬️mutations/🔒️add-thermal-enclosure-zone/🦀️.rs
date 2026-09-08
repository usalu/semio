//! 🔒️ Energy model mutation — `AddThermalEnclosureZone`: Puts one zone inside a thermal enclosure.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔒️ `add-thermal-enclosure-zone` payload. Puts one zone inside a thermal enclosure.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-thermal-enclosure-zone")]
pub struct AddThermalEnclosureZone {
    pub id: crate::model::EntityId,
    pub index: u32,
    pub zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_thermal_enclosure_zone(id: crate::model::EntityId, index: u32, zone_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::AddThermalEnclosureZone(AddThermalEnclosureZone { id, index, zone_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for AddThermalEnclosureZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "thermal-enclosure", kind: "add-thermal-enclosure-zone", record: "AddedThermalEnclosureZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Add zone {} to thermal enclosure {}", self.zone_id.0, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
