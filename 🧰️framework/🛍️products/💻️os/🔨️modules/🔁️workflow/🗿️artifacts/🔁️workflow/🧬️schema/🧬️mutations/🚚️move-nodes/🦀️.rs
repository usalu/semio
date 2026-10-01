use super::super::{WorkflowDiff, WorkflowMutation, WorkflowNodePosition, WorkflowSnapshot};
use super::{workflow_label_number, workflow_targets_invariant};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Payload
/// 🚚️ One node-graph drag as intent (the node-graph gesture record of design §13.3): the canvas offset every addressed
/// workflow node moves by from its BASE position, so editing the drag in history replays it on any base.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "move-nodes")]
pub struct MoveNodes {
    #[dsl(key = "ids")]
    pub node_ids: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}
//#endregion 🔖️Payload

//#region ⚙️Semantics
impl protocol::MutationKind<WorkflowSnapshot, WorkflowMutation> for MoveNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "workflow", kind: "move-nodes", record: "MovedWorkflowNodes" };
    /// 🔺️ A malformed target list or a non-finite offset is `mutation.invariant`; nodes the graph lacks are skipped
    /// (`mutation.partial`), none left is `mutation.target-missing`; a zero offset is `mutation.no-op`. The diff carries
    /// the ABSOLUTE positions the offset lands on.
    fn diff(&self, base: &WorkflowSnapshot) -> protocol::MutationOutcome<WorkflowDiff> {
        if let Err(reason) = workflow_targets_invariant(&self.node_ids) {
            return protocol::MutationOutcome::fatal("mutation.invariant", reason, self.node_ids.clone());
        }
        if !self.dx.is_finite() || !self.dy.is_finite() {
            return protocol::MutationOutcome::fatal("mutation.invariant", "a node offset must be finite", self.node_ids.clone());
        }
        let positions: Vec<WorkflowNodePosition> = self.node_ids.iter().filter_map(|id| base.graph.nodes.iter().find(|node| &node.id == id)).map(|node| WorkflowNodePosition { node_id: node.id.clone(), x: node.x + self.dx, y: node.y + self.dy }).collect();
        let missing: Vec<String> = self.node_ids.iter().filter(|id| !positions.iter().any(|position| &position.node_id == *id)).cloned().collect();
        if positions.is_empty() {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} workflow node(s) exists", self.node_ids.len()), missing);
        }
        let partial = (!missing.is_empty()).then(|| protocol::MutationMessage::warn("mutation.partial", format!("{} of {} node(s) skipped (no such node): {}", missing.len(), self.node_ids.len(), missing.join(", "))).at(missing));
        if (self.dx, self.dy) == (0.0, 0.0) {
            return protocol::MutationOutcome::empty().absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warn("mutation.no-op", "the drag offset is zero").at(self.node_ids.clone())]));
        }
        if positions.iter().any(|position| !position.x.is_finite() || !position.y.is_finite()) {
            return protocol::MutationOutcome::error("mutation.target-mismatch", "the moved position leaves the finite canvas", self.node_ids.clone());
        }
        protocol::MutationOutcome::new(WorkflowDiff::PlaceNodes { positions }).absorb_messages(partial)
    }
    /// ↩️ ONE absolute `set-node-positions` row with every moved node's BASE position — never a negated offset.
    fn inverse(&self, base: &WorkflowSnapshot) -> Vec<WorkflowMutation> {
        if workflow_targets_invariant(&self.node_ids).is_err() || !self.dx.is_finite() || !self.dy.is_finite() || (self.dx, self.dy) == (0.0, 0.0) {
            return Vec::new();
        }
        let positions: Vec<WorkflowNodePosition> = self.node_ids.iter().filter_map(|id| base.graph.nodes.iter().find(|node| &node.id == id)).map(|node| WorkflowNodePosition { node_id: node.id.clone(), x: node.x, y: node.y }).collect();
        if positions.is_empty() {
            return Vec::new();
        }
        vec![WorkflowMutation::SetNodePositions(super::super::SetNodePositions { positions })]
    }
    fn label(&self) -> protocol::LocalizedLabel {
        let [(x_en, x_de), (y_en, y_de)] = [self.dx, self.dy].map(workflow_label_number);
        protocol::LocalizedLabel::native(&format!("Move {} workflow node(s) by ({x_en}, {y_en})", self.node_ids.len()), &format!("{} Arbeitsablaufknoten um ({x_de}; {y_de}) verschieben", self.node_ids.len()))
    }
    fn target(&self) -> Vec<String> {
        std::iter::once("nodes".to_string()).chain(self.node_ids.iter().cloned()).collect()
    }
}
//#endregion ⚙️Semantics

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
