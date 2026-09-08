//! 🔓️ Energy model mutation — `RemoveThermalEnclosureZone`: Takes one zone back out of a thermal enclosure; the zone itself survives.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔓️ `remove-thermal-enclosure-zone` payload. Takes one zone back out of a thermal enclosure; the zone itself survives.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-thermal-enclosure-zone")]
pub struct RemoveThermalEnclosureZone {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_thermal_enclosure_zone(id: crate::model::EntityId, zone_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::RemoveThermalEnclosureZone(RemoveThermalEnclosureZone { id, zone_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RemoveThermalEnclosureZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "thermal-enclosure", kind: "remove-thermal-enclosure-zone", record: "RemovedThermalEnclosureZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Remove zone {} from thermal enclosure {}", self.zone_id.0, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
