//! 🔀️ Move Synapse direct payload and owned behavior.
use super::super::{FlowFixture, FlowDiff, FlowDelta, FlowCollectionDelta, FlowMutation, flow_wire_index};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor, Identified};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, crate::os_dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "move-synapse")]
pub struct MoveSynapse { pub id: String, pub to_index: u32 }

//#endregion 🧬️Payload

//#region 🎮️Behavior
impl MutationKind<FlowFixture, FlowMutation> for MoveSynapse {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "synapse", kind: "move-synapse", record: "MovedSynapse" };
    fn diff(&self, base: &FlowFixture) -> MutationOutcome<FlowDiff> {
        MutationOutcome::new(FlowDiff::from(FlowDelta::Synapses(FlowCollectionDelta { removed: vec![self.id.clone()], inserted: base.synapses.iter().find(|item| item.id() == &self.id).map(|item| (self.to_index, item.clone())).into_iter().collect(), replaced: vec![] })))
    }
    fn inverse(&self, base: &FlowFixture) -> Vec<FlowMutation> {
        base.synapses.iter().position(|item| item.id() == &self.id).and_then(|index| flow_wire_index(index).ok().map(|to_index| FlowMutation::MoveSynapse(Self { id: self.id.clone(), to_index }))).into_iter().collect()
    }
    fn label(&self) -> String { format!("Move synapse {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["synapses".into(), self.id.clone()] }
}

//#endregion 🎮️Behavior

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::flow_direct_tests::assert_leaf_contract::<MoveSynapse>(6, FlowMutation::MoveSynapse, include_str!("🔣️.json")); }
}
//#endregion 🧪️Tests
