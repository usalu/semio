//! 👥️ Replace Presence in the Trinity rewriting presence channel.

use super::{RewritingPresence, RewritingPresenceMutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: RewritingPresence,
}

impl protocol::MutationKind<RewritingPresence, RewritingPresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacePresence" };
    fn diff(&self, _base: &RewritingPresence) -> protocol::MutationOutcome<RewritingPresence> {
        protocol::MutationOutcome::new(self.presence.clone())
    }
    fn inverse(&self, base: &RewritingPresence) -> Vec<RewritingPresenceMutation> { vec![RewritingPresenceMutation::ReplacePresence(ReplacePresence { presence: base.clone() })] }
    fn label(&self) -> String { "Replace Presence".into() }
    fn target(&self) -> Vec<String> { vec!["presence".into()] }
}
