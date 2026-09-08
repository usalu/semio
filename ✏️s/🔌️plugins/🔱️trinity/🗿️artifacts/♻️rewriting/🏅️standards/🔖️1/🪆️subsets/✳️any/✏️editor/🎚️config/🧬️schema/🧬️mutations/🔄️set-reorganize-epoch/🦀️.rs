//! 🔄️ Set Reorganize Epoch in the Trinity rewriting configuration channel.

use super::{RewritingConfig, RewritingConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-reorganize-epoch")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetReorganizeEpoch {
    pub value: u64,
}

impl protocol::MutationKind<RewritingConfig, RewritingConfigMutation> for SetReorganizeEpoch {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "reorganize-epoch", kind: "set-reorganize-epoch", record: "SetReorganizeEpoch" };
    fn diff(&self, base: &RewritingConfig) -> protocol::MutationOutcome<RewritingConfig> {
        let mut next = base.clone();
        next.reorganize_epoch = self.value;
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &RewritingConfig) -> Vec<RewritingConfigMutation> { vec![RewritingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Reorganize Epoch".into() }
    fn target(&self) -> Vec<String> { vec!["reorganize_epoch".into()] }
}
