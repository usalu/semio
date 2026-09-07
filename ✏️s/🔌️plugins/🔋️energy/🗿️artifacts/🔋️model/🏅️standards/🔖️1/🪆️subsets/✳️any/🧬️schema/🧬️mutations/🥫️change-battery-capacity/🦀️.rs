//! 🥫️ Energy model mutation — `ChangeBatteryCapacity`: Sets storage capacity (kWh) on one battery, addressed by id.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🥫️ `change-battery-capacity` payload. Sets storage capacity (kWh) on one battery, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-battery-capacity")]
pub struct ChangeBatteryCapacity {
    pub id: crate::model::EntityId,
    pub new_capacity_kwh: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_battery_capacity(id: crate::model::EntityId, new_capacity_kwh: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeBatteryCapacity(ChangeBatteryCapacity { id, new_capacity_kwh })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeBatteryCapacity {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "battery", kind: "change-battery-capacity", record: "ChangedBatteryCapacity" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Battery Capacity of battery {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
