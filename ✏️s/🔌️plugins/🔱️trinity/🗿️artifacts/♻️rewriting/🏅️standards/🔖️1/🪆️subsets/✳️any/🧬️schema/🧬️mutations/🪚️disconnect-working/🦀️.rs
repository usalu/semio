//! 🪚️ Relative rewriting mutation — `DisconnectWorkingEdges`: a set of working-graph edges cut by id. The cut wires are the
//! payload, so editing the cut in history removes those edges from whatever graph it replays on instead of writing a whole graph
//! back. The node-graph `disconnect` row and the wires a `delete` row names yield it (design §13.3 of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); its exact undo is the base graph's `edit-before-fixture`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 🪚️ `disconnect-working-edges` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "disconnect-working-edges")]
pub struct DisconnectWorkingEdges {
    pub targets: Vec<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn disconnect_working_edges(targets: Vec<String>) -> RewriteRuleMutation {
    RewriteRuleMutation::DisconnectWorkingEdges(DisconnectWorkingEdges { targets })
}

impl DisconnectWorkingEdges {
    /// 🛂️ Whether the payload is well-formed: at least one edge, none twice.
    pub fn holds_invariants(&self) -> bool {
        !self.targets.is_empty() && !self.targets.iter().enumerate().any(|(at, id)| self.targets[..at].contains(id))
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for DisconnectWorkingEdges {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "disconnect", entity: "working-edges", kind: "disconnect-working-edges", record: "DisconnectedWorkingEdges" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match self.targets.len() {
            1 => semio_framework_ui_locale::LocalizedLabel::native("Disconnect 1 edge", "1 Kante trennen"),
            count => semio_framework_ui_locale::LocalizedLabel::native(&format!("Disconnect {count} edges"), &format!("{count} Kanten trennen")),
        }
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation
