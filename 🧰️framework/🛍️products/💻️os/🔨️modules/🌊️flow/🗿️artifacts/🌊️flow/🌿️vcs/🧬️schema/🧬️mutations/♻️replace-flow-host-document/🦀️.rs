//! ♻️ Replace Flow Fixture direct payload and owned behavior.
use super::super::{FlowHostDocument, FlowDiff, FlowDelta, FlowMutation};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
/// 🔮️ First-party Replace Flow Fixture payload.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, crate::os_dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-flow-host-document")]
pub struct ReplaceFlowHostDocument { #[dsl(block)] pub host_document: FlowHostDocument }

//#endregion 🧬️Payload

//#region 🎮️Behavior
impl MutationKind<FlowHostDocument, FlowMutation> for ReplaceFlowHostDocument {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "flow-fixture", kind: "replace-flow-host-document", record: "ReplacedFlowHostDocument" };
    fn diff(&self, _base: &FlowHostDocument) -> MutationOutcome<FlowDiff> {
        MutationOutcome::new(FlowDiff::from(FlowDelta::HostDocument(self.host_document.clone())))
    }
    fn inverse(&self, base: &FlowHostDocument) -> Vec<FlowMutation> {
        vec![FlowMutation::ReplaceFlowHostDocument(Self { host_document: base.clone() })]
    }
    fn label(&self) -> String { "Replace flow fixture".into() }
    fn target(&self) -> Vec<String> { vec![] }
}

//#endregion 🎮️Behavior

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
