//! ✋️ `drag-nodes` — a relative, parametric drag of a set of flow nodes by one common canvas offset. The gesture's own
//! inputs (which nodes, which offset) are the payload, so editing the drag in history re-derives every position from
//! whatever base it replays on. The flow editor's drag tool yields it, one per gesture.

use super::*;
use crate::standards::v1::subsets::base::schema::triples::NamedModified;
use crate::standards::v1::subsets::flow::schema::diff::{FlowNodeDiff, FlowNodesDiff};

//#region 🔖️Payload
/// ✋️ `drag-nodes` payload — the node ids it moves and the offset every one of them moves by.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DragNodes {
    pub targets: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

impl DragNodes {
    /// 🧮️ The drag against `base`: every addressed node moves by `(dx, dy)` read off its BASE position. An empty or
    /// repeated target list or a non-finite offset is a Fatal `mutation.invariant`, no addressed node left is
    /// `mutation.target-missing`, a node the flow lacks is skipped as `mutation.partial`, a zero offset is `mutation.no-op`.
    pub fn outcome(&self, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<SemioFlowDiff> {
        if self.targets.is_empty() || self.targets.iter().enumerate().any(|(at, id)| self.targets[..at].contains(id)) {
            return protocol::MutationOutcome::fatal("mutation.invariant", "targets must name at least one node and never one twice", self.targets.clone());
        }
        if !(self.dx.is_finite() && self.dy.is_finite()) {
            return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", self.targets.clone());
        }
        let moved: Vec<&FlowNode> = self.targets.iter().filter_map(|id| node_at(base, id)).collect();
        if moved.is_empty() {
            return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is a node of this flow", self.targets.len()), self.targets.clone());
        }
        let missing: Vec<String> = self.targets.iter().filter(|id| node_at(base, id).is_none()).cloned().collect();
        let partial: Vec<protocol::MutationMessage> =
            (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped (not in this flow): {}", missing.len(), self.targets.len(), missing.join(", "))).at(missing)).into_iter().collect();
        if (self.dx, self.dy) == (0.0, 0.0) {
            return protocol::MutationOutcome::new(SemioFlowDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "a zero offset moves nothing").at(self.targets.clone())]));
        }
        let modified = moved.into_iter().map(|node| NamedModified { key: node.id.clone(), diff: FlowNodeDiff { position: Some(SemioPoint2 { x: node.position.x + self.dx, y: node.position.y + self.dy }), ..Default::default() } }).collect();
        protocol::MutationOutcome::new(SemioFlowDiff { nodes: Some(FlowNodesDiff { modified, ..Default::default() }), edges: None }).absorb_messages(partial)
    }

    /// ↩️ The exact undo: one `set-node-position` per moved node, carrying its BASE position — never a negated offset.
    pub fn undo(&self, base: &SemioFlowSnapshot) -> Vec<SemioFlowMutation> {
        if self.targets.is_empty() || !(self.dx.is_finite() && self.dy.is_finite()) || (self.dx, self.dy) == (0.0, 0.0) {
            return Vec::new();
        }
        let mut seen = std::collections::HashSet::new();
        self.targets
            .iter()
            .filter(|id| seen.insert(id.as_str()))
            .filter_map(|id| node_at(base, id))
            .map(|node| SemioFlowMutation::SetNodePosition(set_node_position::SetNodePosition { id: node.id.clone(), position: node.position }))
            .collect()
    }
}

impl protocol::MutationKind<SemioFlowSnapshot, SemioFlowMutation> for DragNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "nodes", kind: "drag-nodes", record: "DraggedNodes" };

    fn diff(&self, base: &SemioFlowSnapshot) -> protocol::MutationOutcome<<SemioFlowMutation as Mutation<SemioFlowSnapshot>>::Diff> {
        self.outcome(base)
    }
    fn inverse(&self, base: &SemioFlowSnapshot) -> Result<Vec<SemioFlowMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        self.undo(base)
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let number = |value: f64| {
            let rounded = (value * 100.0).round() / 100.0;
            let text = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded });
            let en = text.trim_end_matches('0').trim_end_matches('.').to_string();
            let de = en.replace('.', ",");
            (en, de)
        };
        let ((dx_en, dx_de), (dy_en, dy_de)) = (number(self.dx), number(self.dy));
        let (items_en, items_de) = match self.targets.len() {
            1 => ("1 node".to_string(), "1 Knoten".to_string()),
            count => (format!("{count} nodes"), format!("{count} Knoten")),
        };
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Drag {items_en} by ({dx_en}, {dy_en})"), &format!("{items_de} um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Payload
