//! ↔️ Move Widget direct payload and owned behavior.
use super::super::{FlowFixture, FlowDiff, FlowDelta, FlowCollectionDelta, FlowMutation, flow_wire_index};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor, Identified};
use serde::{Deserialize, Serialize};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, crate::os_dsl::DslRecord, crate::os_dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "move-widget")]
pub struct MoveWidget { pub id: String, pub to_index: u32 }

//#endregion 🧬️Payload

//#region 🎮️Behavior
impl MutationKind<FlowFixture, FlowMutation> for MoveWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "widget", kind: "move-widget", record: "MovedWidget" };
    fn diff(&self, base: &FlowFixture) -> MutationOutcome<FlowDiff> {
        MutationOutcome::new(FlowDiff::from(FlowDelta::Widgets(FlowCollectionDelta { removed: vec![self.id.clone()], inserted: base.widgets.iter().find(|item| item.id() == &self.id).map(|item| (self.to_index, item.clone())).into_iter().collect(), replaced: vec![] })))
    }
    fn inverse(&self, base: &FlowFixture) -> Vec<FlowMutation> {
        base.widgets.iter().position(|item| item.id() == &self.id).and_then(|index| flow_wire_index(index).ok().map(|to_index| FlowMutation::MoveWidget(Self { id: self.id.clone(), to_index }))).into_iter().collect()
    }
    fn label(&self) -> String { format!("Move widget {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["widgets".into(), self.id.clone()] }
}

//#endregion 🎮️Behavior

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::flow_direct_tests::assert_leaf_contract::<MoveWidget>(2, FlowMutation::MoveWidget, include_str!("🔣️.json")); }
}
//#endregion 🧪️Tests
