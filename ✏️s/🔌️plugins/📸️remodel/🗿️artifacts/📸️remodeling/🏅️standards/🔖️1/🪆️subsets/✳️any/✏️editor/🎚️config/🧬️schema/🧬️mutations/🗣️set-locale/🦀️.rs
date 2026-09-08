//! 🗣️ Set Locale in the remodeling config channel.

use super::{RemodelingConfig, RemodelingConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-locale")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLocale {
    pub value: String,
}

impl protocol::MutationKind<RemodelingConfig, RemodelingConfigMutation> for SetLocale {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "locale", kind: "set-locale", record: "SetLocale" };
    fn diff(&self, base: &RemodelingConfig) -> protocol::MutationOutcome<RemodelingConfig> {
        let mut next = base.clone();
        next.locale = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &RemodelingConfig) -> Vec<RemodelingConfigMutation> { vec![RemodelingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Locale".into() }
    fn target(&self) -> Vec<String> { vec!["locale".into()] }
}
