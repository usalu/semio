use super::super::{WorkflowDiff, WorkflowMutation, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "update-node-ports")]
pub struct UpdateNodePorts {}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for UpdateNodePorts {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "workflow", kind: "update-node-ports", record: "UpdatedWorkflowNodePorts" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> {
        protocol::MutationOutcome::new(WorkflowDiff::SyncNodePorts)
    }
    fn inverse(&self, _base: &WorkflowSnapshot) -> Vec<WorkflowMutation> {
        Vec::new()
    }
    fn label(&self) -> String {
        "Update workflow node ports".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["nodes".into()]
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
