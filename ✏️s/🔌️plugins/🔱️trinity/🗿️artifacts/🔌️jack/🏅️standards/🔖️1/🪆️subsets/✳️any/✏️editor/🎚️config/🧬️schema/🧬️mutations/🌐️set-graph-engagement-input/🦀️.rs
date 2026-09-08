//! 🌐️ Set Graph Engagement Input in the Trinity jack configuration channel.

use super::{JackConfig, JackConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-graph-engagement-input")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetGraphEngagementInput {
    pub value: String,
}

impl protocol::MutationKind<JackConfig, JackConfigMutation> for SetGraphEngagementInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "graph-engagement-input", kind: "set-graph-engagement-input", record: "SetGraphEngagementInput" };
    fn diff(&self, base: &JackConfig) -> protocol::MutationOutcome<JackConfig> {
        let mut next = base.clone();
        next.graph_engagement_input = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &JackConfig) -> Vec<JackConfigMutation> { vec![JackConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Graph Engagement Input".into() }
    fn target(&self) -> Vec<String> { vec!["graph_engagement_input".into()] }
}
