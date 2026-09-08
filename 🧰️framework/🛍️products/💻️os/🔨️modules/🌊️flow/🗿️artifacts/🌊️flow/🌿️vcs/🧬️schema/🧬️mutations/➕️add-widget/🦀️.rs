//! ➕️ Add Widget direct payload and owned behavior.
use super::super::{FlowFixture, FlowDiff, FlowDelta, FlowCollectionDelta, FlowMutation, Widget};
use crate::os_spr::{MutationKind, MutationOutcome, SemanticDescriptor, Identified};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
/// 🔮️ First-party Add Widget payload.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, crate::os_dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "add-widget")]
pub struct AddWidget { pub index: u32, #[dsl(block)] pub widget: Widget }

//#endregion 🧬️Payload

//#region 🎮️Behavior
impl MutationKind<FlowFixture, FlowMutation> for AddWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "add", entity: "widget", kind: "add-widget", record: "AddedWidget" };
    fn diff(&self, _base: &FlowFixture) -> MutationOutcome<FlowDiff> {
        MutationOutcome::new(FlowDiff::from(FlowDelta::Widgets(FlowCollectionDelta { removed: vec![], inserted: vec![(self.index, self.widget.clone())], replaced: vec![] })))
    }
    fn inverse(&self, _base: &FlowFixture) -> Vec<FlowMutation> {
        vec![FlowMutation::RemoveWidget(super::RemoveWidget { id: self.widget.id().clone() })]
    }
    fn label(&self) -> String { format!("Add widget {}", self.widget.id()) }
    fn target(&self) -> Vec<String> { vec!["widgets".into(), self.widget.id().clone()] }
}

//#endregion 🎮️Behavior

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
