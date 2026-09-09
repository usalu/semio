//! 📸️ Replace Config in the Trinity jack configuration channel.

use super::{JackConfig, JackConfigMutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: JackConfig,
}

impl protocol::MutationKind<JackConfig, JackConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, _base: &JackConfig) -> protocol::MutationOutcome<JackConfig> {
        protocol::MutationOutcome::new(self.config.clone())
    }
    fn inverse(&self, base: &JackConfig) -> Vec<JackConfigMutation> {
        vec![JackConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]
    }
    fn label(&self) -> String {
        "Replace Config".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
