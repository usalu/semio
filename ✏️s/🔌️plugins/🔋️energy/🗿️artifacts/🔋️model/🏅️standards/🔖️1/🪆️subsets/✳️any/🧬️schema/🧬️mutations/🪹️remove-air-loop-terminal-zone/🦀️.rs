//! 🪹️ Energy model mutation — `RemoveAirLoopTerminalZone`: Takes one zone off the air loop's terminal list. Refused when the loop does not serve it.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪹️ `remove-air-loop-terminal-zone` payload. Takes one zone off the air loop's terminal list. Refused when the loop does not serve it.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-air-loop-terminal-zone")]
pub struct RemoveAirLoopTerminalZone {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_air_loop_terminal_zone(id: crate::model::EntityId, zone_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::RemoveAirLoopTerminalZone(RemoveAirLoopTerminalZone { id, zone_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for RemoveAirLoopTerminalZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "air-loop", kind: "remove-air-loop-terminal-zone", record: "RemovedAirLoopTerminalZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Remove terminal zone {} from air loop {}", self.zone_id.0, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string(), self.zone_id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
