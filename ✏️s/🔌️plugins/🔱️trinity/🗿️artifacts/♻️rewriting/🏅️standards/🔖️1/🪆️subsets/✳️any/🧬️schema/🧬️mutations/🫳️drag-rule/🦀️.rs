//! 🫳️ Relative rewriting mutation — `DragRuleNodes`: a set of semantic rule-graph nodes (the LHS/RHS clause nodes) moved by
//! one common offset in canvas units, from each node's base position — its `rule_layout` point, else its default slot
//! ([`crate::standards::v1::subsets::any::schema::rule_graph_position`]). The node-graph drag tool yields it, one per released
//! drag (design §13.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); its exact undo is ONE `set-rule-layout-points`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 🫳️ `drag-rule-nodes` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "drag-rule-nodes")]
pub struct DragRuleNodes {
    pub targets: Vec<String>,
    pub dx: f64,
    pub dy: f64,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn drag_rule_nodes(targets: Vec<String>, dx: f64, dy: f64) -> RewriteRuleMutation {
    RewriteRuleMutation::DragRuleNodes(DragRuleNodes { targets, dx, dy })
}

impl DragRuleNodes {
    /// 🛂️ Whether the payload is well-formed: at least one target, none twice, a finite offset.
    pub fn holds_invariants(&self) -> bool {
        !self.targets.is_empty() && !self.targets.iter().enumerate().any(|(at, id)| self.targets[..at].contains(id)) && self.dx.is_finite() && self.dy.is_finite()
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for DragRuleNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "rule-nodes", kind: "drag-rule-nodes", record: "DraggedRuleNodes" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        let ((dx_en, dx_de), (dy_en, dy_de)) = (super::super::offset_text(self.dx), super::super::offset_text(self.dy));
        let (items_en, items_de) = match self.targets.len() {
            1 => ("1 rule node".to_string(), "1 Regelknoten".to_string()),
            count => (format!("{count} rule nodes"), format!("{count} Regelknoten")),
        };
        protocol::LocalizedLabel::native(&format!("Drag {items_en} by ({dx_en}, {dy_en})"), &format!("{items_de} um ({dx_de}; {dy_de}) ziehen"))
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation
