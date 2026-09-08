//! 👥️ Replace Presence in the Trinity jack presence channel.

use super::{JackPresence, JackPresenceMutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "replace-presence")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplacePresence {
    #[dsl(block)]
    pub presence: JackPresence,
}

impl protocol::MutationKind<JackPresence, JackPresenceMutation> for ReplacePresence {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "presence", kind: "replace-presence", record: "ReplacePresence" };
    fn diff(&self, _base: &JackPresence) -> protocol::MutationOutcome<JackPresence> {
        protocol::MutationOutcome::new(self.presence.clone())
    }
    fn inverse(&self, base: &JackPresence) -> Vec<JackPresenceMutation> { vec![JackPresenceMutation::ReplacePresence(ReplacePresence { presence: base.clone() })] }
    fn label(&self) -> String { "Replace Presence".into() }
    fn target(&self) -> Vec<String> { vec!["presence".into()] }
}
