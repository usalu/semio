//! 🥉️ Energy model mutation — `ChangeBatteryRoundTripEfficiency`: Sets round-trip efficiency on one battery, addressed by id.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🥉️ `change-battery-round-trip-efficiency` payload. Sets round-trip efficiency on one battery, addressed by id.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-battery-round-trip-efficiency")]
pub struct ChangeBatteryRoundTripEfficiency {
    pub id: crate::model::EntityId,
    pub new_round_trip_efficiency: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_battery_round_trip_efficiency(id: crate::model::EntityId, new_round_trip_efficiency: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeBatteryRoundTripEfficiency(ChangeBatteryRoundTripEfficiency { id, new_round_trip_efficiency })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeBatteryRoundTripEfficiency {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "battery", kind: "change-battery-round-trip-efficiency", record: "ChangedBatteryRoundTripEfficiency" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change Battery Round Trip Efficiency of battery {}", self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
