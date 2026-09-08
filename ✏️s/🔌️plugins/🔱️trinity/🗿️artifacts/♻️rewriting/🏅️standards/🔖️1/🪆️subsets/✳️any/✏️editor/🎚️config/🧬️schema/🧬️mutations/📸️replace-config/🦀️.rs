//! 📸️ Replace Config in the Trinity rewriting configuration channel.

use super::{RewritingConfig, RewritingConfigMutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: RewritingConfig,
}

impl protocol::MutationKind<RewritingConfig, RewritingConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, _base: &RewritingConfig) -> protocol::MutationOutcome<RewritingConfig> {
        protocol::MutationOutcome::new(self.config.clone())
    }
    fn inverse(&self, base: &RewritingConfig) -> Vec<RewritingConfigMutation> { vec![RewritingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Replace Config".into() }
    fn target(&self) -> Vec<String> { vec!["config".into()] }
}
