//! ✋️ Relative rewriting mutation — `DragWorkingNodes`: a set of working-graph nodes moved by one common offset in canvas
//! units. The gesture's own inputs (which nodes, which offset) are the payload, so editing the drag in history re-derives every
//! position from whatever graph it replays on. The node-graph drag tool yields it, one per released drag (design §13.3 of
//! ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); its exact undo is the base graph's `edit-before-fixture`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// ✋️ `drag-working-nodes` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "drag-working-nodes")]
pub struct DragWorkingNodes {
    pub targets: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_working_nodes(targets: Vec<String>, dx: f64, dy: f64) -> RewriteRuleMutation {
    RewriteRuleMutation::DragWorkingNodes(DragWorkingNodes { targets, dx, dy })
}

impl DragWorkingNodes {
    /// 🛂️ Whether the payload is well-formed: at least one target, none twice, a finite offset.
    pub fn holds_invariants(&self) -> bool {
        !self.targets.is_empty() && !self.targets.iter().enumerate().any(|(at, id)| self.targets[..at].contains(id)) && self.dx.is_finite() && self.dy.is_finite()
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for DragWorkingNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "working-nodes", kind: "drag-working-nodes", record: "DraggedWorkingNodes" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de)) = (super::super::offset_text(self.dx), super::super::offset_text(self.dy));
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
//#endregion 🔖️Mutation
