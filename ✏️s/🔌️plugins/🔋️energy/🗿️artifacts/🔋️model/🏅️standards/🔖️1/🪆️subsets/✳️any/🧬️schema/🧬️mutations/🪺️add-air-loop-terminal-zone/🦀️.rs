//! 🪺️ Energy model mutation — `AddAirLoopTerminalZone`: Puts one more zone on the air loop's terminal list, at its ascending position.

use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🪺️ `add-air-loop-terminal-zone` payload. Puts one more zone on the air loop's terminal list, at its ascending position.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "add-air-loop-terminal-zone")]
pub struct AddAirLoopTerminalZone {
    pub id: crate::model::EntityId,
    pub zone_id: crate::model::EntityId,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn add_air_loop_terminal_zone(id: crate::model::EntityId, zone_id: crate::model::EntityId) -> EnergyModelMutation {
    EnergyModelMutation::AddAirLoopTerminalZone(AddAirLoopTerminalZone { id, zone_id })
}

impl protocol::MutationKind<EnergyModelSnapshot, EnergyModelMutation> for AddAirLoopTerminalZone {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "air-loop", kind: "add-air-loop-terminal-zone", record: "AddedAirLoopTerminalZone" };

    fn diff(&self, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Add terminal zone {} to air loop {}", self.zone_id.0, self.id.0)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.0.to_string(), self.zone_id.0.to_string()]
    }
}
//#endregion 🔖️Mutation
