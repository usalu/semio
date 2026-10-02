//! 🔌️ Relative rewriting mutation — `ConnectWorkingPorts`: ONE edge of `kind` drawn on the working graph from the `source` port to
//! the `target` port (port keys `node@port`, a bare node id for a node-level endpoint). The wire's intent is the payload, so editing
//! it in history redraws it on whatever graph it replays on instead of writing a whole graph back. The node-graph `connect` row
//! yields it (design §13.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING); its exact undo is the base graph's `edit-before-fixture`.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::RewritingSnapshot;

//#region 🔖️Mutation
/// 🔌️ `connect-working-ports` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "connect-working-ports")]
pub struct ConnectWorkingPorts {
    pub source: String,
    pub target: String,
    pub kind: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_working_ports(source: String, target: String, kind: String) -> RewriteRuleMutation {
    RewriteRuleMutation::ConnectWorkingPorts(ConnectWorkingPorts { source, target, kind })
}

impl ConnectWorkingPorts {
    /// 🛂️ Whether the payload is well-formed: two different non-blank endpoints and a non-blank edge kind.
    pub fn holds_invariants(&self) -> bool {
        [&self.source, &self.target, &self.kind].iter().all(|text| !text.trim().is_empty()) && self.source != self.target
    }

    /// 🆔️ The id the drawn edge carries — `source->target` — so the same wire always has the same id.
    pub fn edge_id(&self) -> String {
        format!("{}->{}", self.source, self.target)
    }
}

impl protocol::MutationKind<RewritingSnapshot, RewriteRuleMutation> for ConnectWorkingPorts {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "working-ports", kind: "connect-working-ports", record: "ConnectedWorkingPorts" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Vec<RewriteRuleMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Connect two ports", "Zwei Anschlüsse verbinden")
    }
    fn target(&self) -> Vec<String> {
        vec![self.source.clone(), self.target.clone()]
    }
}
//#endregion 🔖️Mutation
