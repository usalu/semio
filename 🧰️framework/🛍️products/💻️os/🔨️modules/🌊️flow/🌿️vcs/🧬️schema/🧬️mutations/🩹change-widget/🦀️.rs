//! 🩹 Change Widget direct payload and owned behavior.
use super::super::{FlowFixture, FlowDiff, FlowDelta, FlowCollectionDelta, FlowMutation, Widget};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor, Identified};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, crate::os_ToValue, FromValue, dsl::DslRecord, crate::os_dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "change-widget")]
pub struct ChangeWidget { pub id: String, #[dsl(block)] pub widget: Widget }

//#endregion 🧬️Payload

//#region 🎮️Behavior
impl MutationKind<FlowFixture, FlowMutation> for ChangeWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "widget", kind: "change-widget", record: "ChangedWidget" };
    fn diff(&self, _base: &FlowFixture) -> MutationOutcome<FlowDiff> {
        MutationOutcome::new(FlowDiff::from(FlowDelta::Widgets(FlowCollectionDelta { removed: vec![], inserted: vec![], replaced: vec![(self.id.clone(), self.widget.clone())] })))
    }
    fn inverse(&self, base: &FlowFixture) -> Vec<FlowMutation> {
        base.widgets.iter().find(|item| item.id() == &self.id).map(|previous| FlowMutation::ChangeWidget(Self { id: self.widget.id().clone(), widget: previous.clone() })).into_iter().collect()
    }
    fn label(&self) -> String { format!("Change widget {}", self.id) }
    fn target(&self) -> Vec<String> { vec!["widgets".into(), self.id.clone()] }
}

//#endregion 🎮️Behavior

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn direct_leaf_contract() { super::super::super::flow_direct_tests::assert_leaf_contract::<ChangeWidget>(3, FlowMutation::ChangeWidget, include_str!("🔣️.json")); }
}
//#endregion 🧪️Tests
