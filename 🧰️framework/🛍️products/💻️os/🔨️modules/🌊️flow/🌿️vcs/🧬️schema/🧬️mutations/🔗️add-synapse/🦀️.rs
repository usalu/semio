//! 🔗️ Add Synapse direct payload and owned behavior.
use super::super::{FlowFixture, FlowDiff, FlowDelta, FlowCollectionDelta, FlowMutation, SynapseSpec};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor, Identified};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, crate::os_dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-synapse")]
pub struct AddSynapse { pub index: u32, #[dsl(block)] pub synapse: SynapseSpec }

//#endregion 🧬️Payload

//#region 🎮️Behavior
impl MutationKind<FlowFixture, FlowMutation> for AddSynapse {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "synapse", kind: "add-synapse", record: "AddedSynapse" };
    fn diff(&self, _base: &FlowFixture) -> MutationOutcome<FlowDiff> {
        MutationOutcome::new(FlowDiff::from(FlowDelta::Synapses(FlowCollectionDelta { removed: vec![], inserted: vec![(self.index, self.synapse.clone())], replaced: vec![] })))
    }
    fn inverse(&self, _base: &FlowFixture) -> Vec<FlowMutation> {
        vec![FlowMutation::RemoveSynapse(super::RemoveSynapse { id: self.synapse.id().clone() })]
    }
    fn label(&self) -> String { format!("Add synapse {}", self.synapse.id()) }
    fn target(&self) -> Vec<String> { vec!["synapses".into(), self.synapse.id().clone()] }
}

//#endregion 🎮️Behavior

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::flow_direct_tests::assert_leaf_contract::<AddSynapse>(4, FlowMutation::AddSynapse, include_str!("🔣️.json")); }
}
//#endregion 🧪️Tests
