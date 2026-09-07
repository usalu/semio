//! 🧻️ Energy model mutation — `ChangeHumidistatDehumidifyingThrottleRange`: Sets the proportional band the dehumidifying setpoint is approached over, in percent relative humidity.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🧻️ `change-humidistat-dehumidifying-throttle-range` payload. Sets the proportional band the dehumidifying setpoint is approached over, in percent relative humidity.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-humidistat-dehumidifying-throttle-range")]
pub struct ChangeHumidistatDehumidifyingThrottleRange {
    pub id: crate::model::EntityId,
    pub new_dehumidifying_throttle_range: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_humidistat_dehumidifying_throttle_range(id: crate::model::EntityId, new_dehumidifying_throttle_range: f64) -> EnergyModelMutation {
    EnergyModelMutation::ChangeHumidistatDehumidifyingThrottleRange(ChangeHumidistatDehumidifyingThrottleRange { id, new_dehumidifying_throttle_range })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for ChangeHumidistatDehumidifyingThrottleRange {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "humidistat", kind: "change-humidistat-dehumidifying-throttle-range", record: "ChangedHumidistatDehumidifyingThrottleRange" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change humidistat {} dehumidifying throttle range to {:?}", self.id.0, self.new_dehumidifying_throttle_range)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
