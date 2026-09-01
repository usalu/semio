//! ♻️ Replace Flow Fixture direct payload and owned behavior.
use super::super::{FlowFixture, FlowDiff, FlowDelta, FlowMutation};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::DslRecord, crate::os_dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-flow-fixture")]
pub struct ReplaceFlowFixture { #[dsl(block)] pub fixture: FlowFixture }

//#endregion 🧬️Payload

//#region 🎮️Behavior
impl MutationKind<FlowFixture, FlowMutation> for ReplaceFlowFixture {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "flow-fixture", kind: "replace-flow-fixture", record: "ReplacedFlowFixture" };
    fn diff(&self, _base: &FlowFixture) -> MutationOutcome<FlowDiff> {
        MutationOutcome::new(FlowDiff::from(FlowDelta::Fixture(self.fixture.clone())))
    }
    fn inverse(&self, base: &FlowFixture) -> Vec<FlowMutation> {
        vec![FlowMutation::ReplaceFlowFixture(Self { fixture: base.clone() })]
    }
    fn label(&self) -> String { "Replace flow fixture".into() }
    fn target(&self) -> Vec<String> { vec![] }
}

//#endregion 🎮️Behavior

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::flow_direct_tests::assert_leaf_contract::<ReplaceFlowFixture>(9, FlowMutation::ReplaceFlowFixture, include_str!("🔣️.json")); }
}
//#endregion 🧪️Tests
