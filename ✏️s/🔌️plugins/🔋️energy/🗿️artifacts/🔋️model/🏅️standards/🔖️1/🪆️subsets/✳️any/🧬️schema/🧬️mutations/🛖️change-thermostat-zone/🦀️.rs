//! 🛖️ Energy model mutation — `ChangeThermostatZone`: Moves one thermostat to another zone. A foreign key, not a nesting field — zones do not nest, so this is a plain reassignment under the general axis and not a `move-to-<container>`.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛖️ `change-thermostat-zone` payload. Moves one thermostat to another zone. A foreign key, not a nesting field — zones do not nest, so this is a plain reassignment under the general axis and not a `move-to-<container>`.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-thermostat-zone")]
pub struct ChangeThermostatZone {
    pub id: crate::model::EntityId,
    pub new_zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_thermostat_zone(id: crate::model::EntityId, new_zone_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::ChangeThermostatZone(ChangeThermostatZone { id, new_zone_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeThermostatZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "thermostat", kind: "change-thermostat-zone", record: "ChangedThermostatZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change thermostat {} zone to {}", self.id.0, self.new_zone_id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
