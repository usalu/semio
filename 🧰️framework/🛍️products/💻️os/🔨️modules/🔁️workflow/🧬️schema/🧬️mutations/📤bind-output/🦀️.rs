use super::super::{UnbindOutput, WorkflowDiff, WorkflowMutation, WorkflowOutputBinding, WorkflowSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "bind-output")]
pub struct BindOutput { pub binding: WorkflowOutputBinding }
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for BindOutput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "bind", entity: "workflow", kind: "bind-output", record: "BoundWorkflowOutput" };
    fn diff(&self, _base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> { protocol::MutationOutcome::new(WorkflowDiff::BindOutput { binding: self.binding.clone() }) }
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> { match base.output_bindings.iter().find(|entry| entry.node_id == self.binding.node_id && entry.port_id == self.binding.port_id) { Some(existing) => vec![WorkflowMutation::BindOutput(BindOutput { binding: existing.clone() })], None => vec![WorkflowMutation::UnbindOutput(UnbindOutput { node_id: self.binding.node_id.clone(), port_id: self.binding.port_id.clone() })] } }
    fn label(&self) -> String { format!("Bind workflow output {}", self.binding.node_id) }
    fn target(&self) -> Vec<String> { vec!["output-bindings".into(), self.binding.node_id.clone(), self.binding.port_id.clone()] }
}
//#endregion ⚙️Semantics

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationLeaf;
    #[test]
    fn metadata_has_the_canonical_identity() { assert_eq!(<BindOutput as MutationLeaf>::DESCRIPTOR.semantic_kind, "bind-output"); }
}
