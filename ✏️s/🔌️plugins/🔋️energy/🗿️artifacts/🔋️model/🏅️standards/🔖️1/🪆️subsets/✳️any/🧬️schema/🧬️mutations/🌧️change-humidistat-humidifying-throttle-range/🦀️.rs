//! 🌧️ Energy model mutation — `ChangeHumidistatHumidifyingThrottleRange`: Sets the proportional band the humidifying setpoint is approached over, in percent relative humidity.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🌧️ `change-humidistat-humidifying-throttle-range` payload. Sets the proportional band the humidifying setpoint is approached over, in percent relative humidity.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-humidistat-humidifying-throttle-range")]
pub struct ChangeHumidistatHumidifyingThrottleRange {
    pub id: crate::model::EntityId,
    pub new_humidifying_throttle_range: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_humidistat_humidifying_throttle_range(id: crate::model::EntityId, new_humidifying_throttle_range: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeHumidistatHumidifyingThrottleRange(ChangeHumidistatHumidifyingThrottleRange { id, new_humidifying_throttle_range })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeHumidistatHumidifyingThrottleRange {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "humidistat", kind: "change-humidistat-humidifying-throttle-range", record: "ChangedHumidistatHumidifyingThrottleRange" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change humidistat {} humidifying throttle range to {:?}", self.id.0, self.new_humidifying_throttle_range)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
