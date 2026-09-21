//! 🔗️ Add Synapse direct payload and owned behavior.
use super::super::{FlowHostSnapshot, FlowDiff, FlowDelta, FlowCollectionDelta, FlowMutation, SynapseSpec};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor, Identified};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, crate::os_dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-synapse")]
pub struct AddSynapse { pub index: u32, #[dsl(block)] pub synapse: SynapseSpec }

//#endregion 🧬️Payload

//#region 🎮️Behavior
impl MutationKind<FlowHostSnapshot, FlowMutation> for AddSynapse {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "synapse", kind: "add-synapse", record: "AddedSynapse" };
    fn diff(&self, _base: &FlowHostSnapshot) -> MutationOutcome<FlowDiff> {
        MutationOutcome::new(FlowDiff::from(FlowDelta::Synapses(FlowCollectionDelta { removed: vec![], inserted: vec![(self.index, self.synapse.clone())], replaced: vec![] })))
    }
    fn inverse(&self, _base: &FlowHostSnapshot) -> Vec<FlowMutation> {
        vec![FlowMutation::RemoveSynapse(super::RemoveSynapse { id: self.synapse.id().clone() })]
    }
    fn label(&self) -> crate::LocalizedLabel {
        crate::LocalizedLabel::native(&format!("Add synapse {}", self.synapse.id()), &format!("Synapse {} hinzufügen", self.synapse.id()))
    }
    fn target(&self) -> Vec<String> { vec!["synapses".into(), self.synapse.id().clone()] }
}

//#endregion 🎮️Behavior

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
