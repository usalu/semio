//! 🔄 Change Synapse direct payload and owned behavior.
use super::super::{FlowFixture, FlowDiff, FlowDelta, FlowCollectionDelta, FlowMutation, SynapseSpec};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor, Identified};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, crate::os_dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-synapse")]
pub struct ChangeSynapse { pub id: String, #[dsl(block)] pub synapse: SynapseSpec }

//#endregion 🧬️Payload

//#region 🎮️Behavior
impl MutationKind<FlowFixture, FlowMutation> for ChangeSynapse {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "synapse", kind: "change-synapse", record: "ChangedSynapse" };
    fn diff(&self, _base: &FlowFixture) -> MutationOutcome<FlowDiff> {
        MutationOutcome::new(FlowDiff::from(FlowDelta::Synapses(FlowCollectionDelta { removed: vec![], inserted: vec![], replaced: vec![(self.id.clone(), self.synapse.clone())] })))
    }
    fn inverse(&self, base: &FlowFixture) -> Vec<FlowMutation> {
        base.synapses.iter().find(|item| item.id() == &self.id).map(|previous| FlowMutation::ChangeSynapse(Self { id: self.synapse.id().clone(), synapse: previous.clone() })).into_iter().collect()
    }
    fn label(&self) -> String { format!("Change synapse {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["synapses".into(), self.id.clone()] }
}

//#endregion 🎮️Behavior

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::flow_direct_tests::assert_leaf_contract::<ChangeSynapse>(7, FlowMutation::ChangeSynapse, include_str!("🔣️.json")); }
}
//#endregion 🧪️Tests
