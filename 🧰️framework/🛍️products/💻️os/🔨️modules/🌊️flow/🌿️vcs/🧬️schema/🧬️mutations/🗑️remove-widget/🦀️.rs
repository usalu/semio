//! 🗑️ Remove Widget direct payload and owned behavior.
use super::super::{FlowFixture, FlowDiff, FlowDelta, FlowCollectionDelta, FlowMutation, flow_wire_index};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor, Identified};
use serde::{Deserialize, Serialize};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, crate::os_dsl::DslRecord, crate::os_dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "remove-widget")]
pub struct RemoveWidget { pub id: String }

//#endregion 🧬️Payload

//#region 🎮️Behavior
impl MutationKind<FlowFixture, FlowMutation> for RemoveWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "widget", kind: "remove-widget", record: "RemovedWidget" };
    fn diff(&self, _base: &FlowFixture) -> MutationOutcome<FlowDiff> {
        MutationOutcome::new(FlowDiff::from(FlowDelta::Widgets(FlowCollectionDelta { removed: vec![self.id.clone()], inserted: vec![], replaced: vec![] })))
    }
    fn inverse(&self, base: &FlowFixture) -> Vec<FlowMutation> {
        base.widgets.iter().position(|item| item.id() == &self.id).and_then(|index| flow_wire_index(index).ok().map(|wire| FlowMutation::AddWidget(super::AddWidget { index: wire, widget: base.widgets[index].clone() }))).into_iter().collect()
    }
    fn label(&self) -> String { format!("Remove widget {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["widgets".into(), self.id.clone()] }
}

//#endregion 🎮️Behavior

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::flow_direct_tests::assert_leaf_contract::<RemoveWidget>(1, FlowMutation::RemoveWidget, include_str!("🔣️.json")); }
}
//#endregion 🧪️Tests
