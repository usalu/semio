use super::super::{WorkflowDiff, WorkflowMediaPort, WorkflowMutation, WorkflowSnapshot};
use super::workflow_targets_invariant;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
/// 🔌️ One workflow node's declared input ports and the node height they imply, installed in ONE row (positional: the ports keep their list order).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkflowNodePorts {
    #[dsl(key = "id")]
    pub node_id: String,
    pub inputs: Vec<WorkflowMediaPort>,
    pub height: f64,
}

/// 🔧️ Installs the input ports and height of a set of workflow nodes: the exact undo of an `update-node-ports` resync (every resynced node back
/// at its base ports) and of itself, so a multi-node port change stays one point-invertible row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-node-ports")]
pub struct SetNodePorts {
    pub nodes: Vec<WorkflowNodePorts>,
}

impl SetNodePorts {
    /// 🆔️ The addressed node ids, in payload order.
    pub fn node_ids(&self) -> Vec<String> {
        self.nodes.iter().map(|row| row.node_id.clone()).collect()
    }
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for SetNodePorts {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "workflow", kind: "set-node-ports", record: "SetWorkflowNodePorts" };


    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set the ports of {} workflow node(s)", self.nodes.len()), &format!("Anschlüsse von {} Arbeitsablaufknoten setzen", self.nodes.len()))
    }
    fn target(&self) -> Vec<String> {
        std::iter::once("nodes".to_string()).chain(self.node_ids()).collect()
    }
}

impl protocol::ColdMutationKind<WorkflowSnapshot, WorkflowMutation> for SetNodePorts {
    /// 🔺️ A malformed row list is `mutation.invariant`; nodes the graph lacks are skipped (`mutation.partial`), none left is `mutation.target-missing`;
    /// every node already carrying its ports and height is `mutation.no-op`.
    fn diff(&self, base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> {
        let ids = self.node_ids();
        if let Err(reason) = workflow_targets_invariant(&ids) {
            return protocol::MutationOutcome::fatal("mutation.invariant", reason, ids);
        }
        if self.nodes.iter().any(|row| !row.height.is_finite()) {
            return protocol::MutationOutcome::fatal("mutation.invariant", "a node height must be finite", ids);
        }
        let present: Vec<&WorkflowNodePorts> = self.nodes.iter().filter(|row| base.graph.nodes.iter().any(|node| node.id == row.node_id)).collect();
        let missing: Vec<String> = self.nodes.iter().filter(|row| !present.iter().any(|kept| kept.node_id == row.node_id)).map(|row| row.node_id.clone()).collect();
        if present.is_empty() {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} workflow node(s) exists", ids.len()), missing);
        }
        let partial = (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} node(s) skipped (no such node): {}", missing.len(), ids.len(), missing.join(", "))).at(missing));
        let changes = present.iter().any(|row| base.graph.nodes.iter().any(|node| node.id == row.node_id && (node.inputs != row.inputs || node.height != row.height)));
        if !changes {
            return protocol::MutationOutcome::empty().absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "every node already carries its ports").at(ids)]));
        }
        protocol::MutationOutcome::new(WorkflowDiff::SetNodePorts { nodes: present.into_iter().cloned().collect() }).absorb_messages(partial)
    }
    /// ↩️ ONE `set-node-ports` row with every addressed node's BASE ports and height; a row set that changes nothing has none.
    fn inverse(&self, base: &WorkflowSnapshot) -> Result<Vec<WorkflowMutation>, semio_framework_value::ValueError> {
        if workflow_targets_invariant(&self.node_ids()).is_err() || self.nodes.iter().any(|row| !row.height.is_finite()) {
            return Ok(Vec::new());
        }
        let placed: Vec<(WorkflowNodePorts, bool)> = self
            .nodes
            .iter()
            .filter_map(|row| base.graph.nodes.iter().find(|node| node.id == row.node_id).map(|node| (WorkflowNodePorts { node_id: node.id.clone(), inputs: node.inputs.clone(), height: node.height }, node.inputs != row.inputs || node.height != row.height)))
            .collect();
        if !placed.iter().any(|(_, changes)| *changes) {
            return Ok(Vec::new());
        }
        Ok(vec![WorkflowMutation::SetNodePorts(SetNodePorts { nodes: placed.into_iter().map(|(row, _)| row).collect() })])
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
