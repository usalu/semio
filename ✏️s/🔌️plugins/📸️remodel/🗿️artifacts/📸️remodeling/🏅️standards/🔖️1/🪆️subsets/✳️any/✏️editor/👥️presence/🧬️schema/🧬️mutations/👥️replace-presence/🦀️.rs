//! 👥️ Replace Presence in the remodeling presence channel.

use super::{RemodelingPresence, RemodelingPresenceMutation};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: RemodelingPresence,
}

impl protocol::MutationKind<RemodelingPresence, RemodelingPresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacePresence" };
    fn diff(&self, _base: &RemodelingPresence) -> protocol::MutationOutcome<RemodelingPresence> {
        protocol::MutationOutcome::new(self.presence.clone())
    }
    fn inverse(&self, base: &RemodelingPresence) -> Vec<RemodelingPresenceMutation> {
        vec![RemodelingPresenceMutation::ReplacePresence(ReplacePresence { presence: base.clone() })]
    }
    fn label(&self) -> String {
        "Replace Presence".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["presence".into()]
    }
}
