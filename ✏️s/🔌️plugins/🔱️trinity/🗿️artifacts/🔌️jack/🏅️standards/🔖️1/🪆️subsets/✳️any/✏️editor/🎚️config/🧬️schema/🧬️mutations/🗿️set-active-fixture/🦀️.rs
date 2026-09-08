//! 🗿️ Set Active Fixture in the Trinity jack configuration channel.

use super::{JackConfig, JackConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-active-fixture")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetActiveFixture {
    pub value: String,
}

impl protocol::MutationKind<JackConfig, JackConfigMutation> for SetActiveFixture {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "active-fixture", kind: "set-active-fixture", record: "SetActiveFixture" };
    fn diff(&self, base: &JackConfig) -> protocol::MutationOutcome<JackConfig> {
        let mut next = base.clone();
        next.active_fixture_id = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &JackConfig) -> Vec<JackConfigMutation> { vec![JackConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Active Fixture".into() }
    fn target(&self) -> Vec<String> { vec!["active_fixture_id".into()] }
}
