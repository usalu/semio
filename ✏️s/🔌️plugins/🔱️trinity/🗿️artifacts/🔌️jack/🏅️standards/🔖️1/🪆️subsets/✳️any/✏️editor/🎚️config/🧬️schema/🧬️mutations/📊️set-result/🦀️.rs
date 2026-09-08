//! 📊️ Set Result in the Trinity jack configuration channel.

use super::{JackConfig, JackConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-result")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetResult {
    pub value: String,
}

impl protocol::MutationKind<JackConfig, JackConfigMutation> for SetResult {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "result", kind: "set-result", record: "SetResult" };
    fn diff(&self, base: &JackConfig) -> protocol::MutationOutcome<JackConfig> {
        let mut next = base.clone();
        next.jack_result_json = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &JackConfig) -> Vec<JackConfigMutation> { vec![JackConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Result".into() }
    fn target(&self) -> Vec<String> { vec!["jack_result_json".into()] }
}
