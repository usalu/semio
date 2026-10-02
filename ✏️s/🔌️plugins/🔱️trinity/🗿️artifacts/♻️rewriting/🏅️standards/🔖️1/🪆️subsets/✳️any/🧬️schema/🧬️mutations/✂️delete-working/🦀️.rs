//! ✂️ Relative rewriting mutation — `DeleteWorkingNodes`: a set of working-graph nodes removed together with every edge that
//! touches one of them. The selection is the payload, so editing the delete in history removes those nodes from whatever graph
//! it replays on instead of writing a whole graph back. The node-graph delete gesture yields it (design §13.3 of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); its exact undo is the base graph's `edit-before-fixture`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// ✂️ `delete-working-nodes` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-working-nodes")]
pub struct DeleteWorkingNodes {
    pub targets: Vec<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_working_nodes(targets: Vec<String>) -> RewriteRuleMutation {
    RewriteRuleMutation::DeleteWorkingNodes(DeleteWorkingNodes { targets })
}

impl DeleteWorkingNodes {
    /// 🛂️ Whether the payload is well-formed: at least one target, none twice.
    pub fn holds_invariants(&self) -> bool {
        !self.targets.is_empty() && !self.targets.iter().enumerate().any(|(at, id)| self.targets[..at].contains(id))
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for DeleteWorkingNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "working-nodes", kind: "delete-working-nodes", record: "DeletedWorkingNodes" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        match self.targets.len() {
            1 => semio_framework_ui_locale::LocalizedLabel::native("Delete 1 node", "1 Knoten löschen"),
            count => semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete {count} nodes"), &format!("{count} Knoten löschen")),
        }
    }
    fn target(&self) -> Vec<String> {
        self.targets.clone()
    }
}
//#endregion 🔖️Mutation
