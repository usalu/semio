use super::super::{sync_workflow_parameter_ports, WorkflowDiff, WorkflowMutation, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "update-node-ports")]
pub struct UpdateNodePorts {}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for UpdateNodePorts {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "workflow", kind: "update-node-ports", record: "UpdatedWorkflowNodePorts" };
    fn diff(&self, base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> {
        if sync_workflow_parameter_ports(&base.graph, &base.parameter_bindings) == base.graph {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "node ports already match their parameter bindings");
        }
        protocol::MutationOutcome::new(WorkflowDiff::SyncNodePorts)
    }
    fn inverse(&self, base: &WorkflowSnapshot) -> Result<Vec<WorkflowMutation>, semio_framework_value::ValueError> {
        if sync_workflow_parameter_ports(&base.graph, &base.parameter_bindings) == base.graph {
            return Ok(Vec::new());
        }
        Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "update-node-ports on desynced node ports has no concrete inverse: no kind restores the prior port lists"))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Update workflow node ports", "Arbeitsablaufknotenanschlüsse aktualisieren")
    }
    fn target(&self) -> Vec<String> {
        vec!["nodes".into()]
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
