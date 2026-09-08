//! 🔢️ Set Revision in the Trinity jack configuration channel.

use super::{JackConfig, JackConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-revision")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetRevision {
    pub value: u64,
}

impl protocol::MutationKind<JackConfig, JackConfigMutation> for SetRevision {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "revision", kind: "set-revision", record: "SetRevision" };
    fn diff(&self, base: &JackConfig) -> protocol::MutationOutcome<JackConfig> {
        let mut next = base.clone();
        next.revision = self.value;
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &JackConfig) -> Vec<JackConfigMutation> { vec![JackConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Revision".into() }
    fn target(&self) -> Vec<String> { vec!["revision".into()] }
}
