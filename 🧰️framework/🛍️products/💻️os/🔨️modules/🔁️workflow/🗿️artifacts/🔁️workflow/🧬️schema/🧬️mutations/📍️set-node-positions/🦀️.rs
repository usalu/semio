use super::super::{WorkflowDiff, WorkflowMutation, WorkflowSnapshot};
use super::workflow_targets_invariant;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
/// 📌️ One workflow node's absolute canvas position.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkflowNodePosition {
    #[dsl(key = "id")]
    pub node_id: String,
    pub x: f64,
    pub y: f64,
}

/// 📍️ Absolute canvas positions of a set of workflow nodes in ONE row: the exact undo of a `move-nodes` drag (every moved
/// node back at its base position) and of itself, so a multi-node gesture stays one point-invertible row.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-node-positions")]
pub struct SetNodePositions {
    pub positions: Vec<WorkflowNodePosition>,
}

impl SetNodePositions {
    /// 🆔️ The positioned node ids, in payload order.
    pub fn node_ids(&self) -> Vec<String> {
        self.positions.iter().map(|position| position.node_id.clone()).collect()
    }
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for SetNodePositions {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "workflow", kind: "set-node-positions", record: "SetWorkflowNodePositions" };
    /// 🔺️ A malformed position list is `mutation.invariant`; nodes the graph lacks are skipped (`mutation.partial`), none
    /// left is `mutation.target-missing`; every node already in place is `mutation.no-op`.
    fn diff(&self, base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> {
        let ids = self.node_ids();
        if let Err(reason) = workflow_targets_invariant(&ids) {
            return protocol::MutationOutcome::fatal("mutation.invariant", reason, ids);
        }
        if self.positions.iter().any(|position| !position.x.is_finite() || !position.y.is_finite()) {
            return protocol::MutationOutcome::fatal("mutation.invariant", "a node position must be finite", ids);
        }
        let present: Vec<&WorkflowNodePosition> = self.positions.iter().filter(|position| base.graph.nodes.iter().any(|node| node.id == position.node_id)).collect();
        let missing: Vec<String> = self.positions.iter().filter(|position| !present.iter().any(|kept| kept.node_id == position.node_id)).map(|position| position.node_id.clone()).collect();
        if present.is_empty() {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} workflow node(s) exists", ids.len()), missing);
        }
        let partial = (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} node(s) skipped (no such node): {}", missing.len(), ids.len(), missing.join(", "))).at(missing));
        let moves = present.iter().any(|position| base.graph.nodes.iter().any(|node| node.id == position.node_id && (node.x, node.y) != (position.x, position.y)));
        if !moves {
            return protocol::MutationOutcome::empty().absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "every node already sits at its position").at(ids)]));
        }
        protocol::MutationOutcome::new(WorkflowDiff::PlaceNodes { positions: present.into_iter().cloned().collect() }).absorb_messages(partial)
    }
    /// ↩️ ONE `set-node-positions` row with every placed node's BASE position; a placement that moves nothing has none.
    fn inverse(&self, base: &WorkflowSnapshot) -> Result<Vec<WorkflowMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        if workflow_targets_invariant(&self.node_ids()).is_err() || self.positions.iter().any(|position| !position.x.is_finite() || !position.y.is_finite()) {
            return Vec::new();
        }
        let placed: Vec<(WorkflowNodePosition, bool)> = self
            .positions
            .iter()
            .filter_map(|position| base.graph.nodes.iter().find(|node| node.id == position.node_id).map(|node| (WorkflowNodePosition { node_id: node.id.clone(), x: node.x, y: node.y }, (node.x, node.y) != (position.x, position.y))))
            .collect();
        if !placed.iter().any(|(_, moves)| *moves) {
            return Vec::new();
        }
        vec![WorkflowMutation::SetNodePositions(SetNodePositions { positions: placed.into_iter().map(|(position, _)| position).collect() })]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set the positions of {} workflow node(s)", self.positions.len()), &format!("Positionen von {} Arbeitsablaufknoten setzen", self.positions.len()))
    }
    fn target(&self) -> Vec<String> {
        std::iter::once("nodes".to_string()).chain(self.node_ids()).collect()
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
