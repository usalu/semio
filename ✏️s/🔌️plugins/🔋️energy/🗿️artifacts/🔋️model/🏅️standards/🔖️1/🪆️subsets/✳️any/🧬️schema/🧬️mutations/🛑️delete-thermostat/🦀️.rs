//! 🛑️ Energy model mutation — `DeleteThermostat`: Drops one zone's setpoint control, leaving the zone free-floating. Refused when no thermostat carries the id, so an undo chain can never invent a deletion that had no partner.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🛑️ `delete-thermostat` payload. Drops one zone's setpoint control, leaving the zone free-floating. Refused when no thermostat carries the id, so an undo chain can never invent a deletion that had no partner.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-thermostat")]
pub struct DeleteThermostat {
    pub id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_thermostat(id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::DeleteThermostat(DeleteThermostat { id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for DeleteThermostat {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "thermostat", kind: "delete-thermostat", record: "DeletedThermostat" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete thermostat {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
