//! 📸️ Replace Config in the remodeling config channel.

use super::{RemodelingConfig, RemodelingConfigMutation};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: RemodelingConfig,
}

impl protocol::MutationKind<RemodelingConfig, RemodelingConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, _base: &RemodelingConfig) -> protocol::MutationOutcome<RemodelingConfig> {
        protocol::MutationOutcome::new(self.config.clone())
    }
    fn inverse(&self, base: &RemodelingConfig) -> Vec<RemodelingConfigMutation> { vec![RemodelingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Replace Config".into() }
    fn target(&self) -> Vec<String> { vec!["config".into()] }
}
