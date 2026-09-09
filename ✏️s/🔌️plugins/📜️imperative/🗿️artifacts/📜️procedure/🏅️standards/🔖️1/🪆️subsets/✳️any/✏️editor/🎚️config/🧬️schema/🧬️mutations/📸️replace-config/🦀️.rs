//! 🧬️ Replace Config in the imperative.config channel.

use super::*;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "replace-config")]
#[mutation_leaf(contract = ::protocol)]
pub struct ReplaceConfig {
    #[dsl(block)]
    pub config: ImperativeConfig,
}

impl protocol::MutationKind<ImperativeConfig, ImperativeConfigMutation> for ReplaceConfig {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "config", kind: "replace-config", record: "ReplaceConfig" };
    fn diff(&self, base: &ImperativeConfig) -> protocol::MutationOutcome<ImperativeConfig> {
        if *base == self.config {
            return protocol::MutationOutcome::empty().warn("mutation.no-op", "The requested configuration value is already current.");
        }
        protocol::MutationOutcome::new(self.config.clone())
    }
    fn inverse(&self, base: &ImperativeConfig) -> Vec<ImperativeConfigMutation> {
        vec![ImperativeConfigMutation::ReplaceConfig(Self { config: base.clone() })]
    }
    fn label(&self) -> String {
        "Replace Config".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["config".into()]
    }
}
