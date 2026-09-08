//! 🔄️ Set Reorganize Epoch in the Trinity jack configuration channel.

use super::{JackConfig, JackConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-reorganize-epoch")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetReorganizeEpoch {
    pub value: u64,
}

impl protocol::MutationKind<JackConfig, JackConfigMutation> for SetReorganizeEpoch {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "reorganize-epoch", kind: "set-reorganize-epoch", record: "SetReorganizeEpoch" };
    fn diff(&self, base: &JackConfig) -> protocol::MutationOutcome<JackConfig> {
        let mut next = base.clone();
        next.reorganize_epoch = self.value;
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &JackConfig) -> Vec<JackConfigMutation> { vec![JackConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Reorganize Epoch".into() }
    fn target(&self) -> Vec<String> { vec!["reorganize_epoch".into()] }
}
