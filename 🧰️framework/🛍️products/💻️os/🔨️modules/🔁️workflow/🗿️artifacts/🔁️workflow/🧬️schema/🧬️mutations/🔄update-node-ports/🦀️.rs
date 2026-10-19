use super::super::{sync_workflow_parameter_ports, WorkflowDiff, WorkflowMutation, WorkflowNode, WorkflowSnapshot};
use super::{SetNodePorts, WorkflowNodePorts};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "update-node-ports")]
pub struct UpdateNodePorts {}
//#endregion 🔖️Payload

//#region ⚙️Semantics
/// 🎛️ The nodes whose parameter-bound input ports (and the height those imply) differ from what their parameter bindings derive: each pairs the
/// base node with the row that installs the derived ports. The derivation is an algorithm run on READS of the base; no snapshot is rebuilt or applied.
fn stale_node_ports(base: &WorkflowSnapshot) -> Vec<(&WorkflowNode, WorkflowNodePorts)> {
    let derived = sync_workflow_parameter_ports(&base.graph, &base.parameter_bindings);
    base.graph
        .nodes
        .iter()
        .zip(derived.nodes)
        .filter(|(node, synced)| node.inputs != synced.inputs || node.height != synced.height)
        .map(|(node, synced)| (node, WorkflowNodePorts { node_id: synced.id, inputs: synced.inputs, height: synced.height }))
        .collect()
}

impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for UpdateNodePorts {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "workflow", kind: "update-node-ports", record: "UpdatedWorkflowNodePorts" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Update workflow node ports", "Arbeitsablaufknotenanschlüsse aktualisieren")
    }
    fn target(&self) -> Vec<String> {
        vec!["nodes".into()]
    }
}

impl protocol::ColdMutationKind<WorkflowSnapshot, WorkflowMutation> for UpdateNodePorts {
    /// 🔺️ One `{node, ports, height}` row per node whose ports are stale; nothing stale is `mutation.no-op`.
    fn diff(&self, base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> {
        let rows: Vec<WorkflowNodePorts> = stale_node_ports(base).into_iter().map(|(_, row)| row).collect();
        if rows.is_empty() {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "node ports already match their parameter bindings");
        }
        protocol::MutationOutcome::new(WorkflowDiff::SetNodePorts { nodes: rows })
    }
    /// ↩️ ONE `set-node-ports` row with every resynced node's BASE ports and height, in node order; a resync that changes nothing has none.
    fn inverse(&self, base: &WorkflowSnapshot) -> Result<Vec<WorkflowMutation>, semio_framework_value::ValueError> {
        let stale = stale_node_ports(base);
        if stale.is_empty() {
            return Ok(Vec::new());
        }
        let nodes = stale.into_iter().map(|(node, _)| WorkflowNodePorts { node_id: node.id.clone(), inputs: node.inputs.clone(), height: node.height }).collect();
        Ok(vec![WorkflowMutation::SetNodePorts(SetNodePorts { nodes })])
    }
}
//#endregion ⚙️Semantics
