//! 🔎️ Set Query in the Trinity jack configuration channel.

use super::{JackConfig, JackConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-query")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetQuery {
    pub value: String,
}

impl protocol::MutationKind<JackConfig, JackConfigMutation> for SetQuery {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "query", kind: "set-query", record: "SetQuery" };
    fn diff(&self, base: &JackConfig) -> protocol::MutationOutcome<JackConfig> {
        let mut next = base.clone();
        next.jack_query = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &JackConfig) -> Vec<JackConfigMutation> {
        vec![JackConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]
    }
    fn label(&self) -> String {
        "Set Query".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["jack_query".into()]
    }
}
