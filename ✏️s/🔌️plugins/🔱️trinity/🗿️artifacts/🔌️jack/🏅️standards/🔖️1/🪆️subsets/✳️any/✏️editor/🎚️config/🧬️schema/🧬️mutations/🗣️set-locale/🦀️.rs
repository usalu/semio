//! 🗣️ Set Locale in the Trinity jack configuration channel.

use super::{JackConfig, JackConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-locale")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLocale {
    pub value: String,
}

impl protocol::MutationKind<JackConfig, JackConfigMutation> for SetLocale {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "locale", kind: "set-locale", record: "SetLocale" };
    fn diff(&self, base: &JackConfig) -> protocol::MutationOutcome<JackConfig> {
        let mut next = base.clone();
        next.locale = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &JackConfig) -> Vec<JackConfigMutation> { vec![JackConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Locale".into() }
    fn target(&self) -> Vec<String> { vec!["locale".into()] }
}
