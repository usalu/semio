//! 📈️ Set Results Engagement Input in the Trinity jack configuration channel.

use super::{JackConfig, JackConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-results-engagement-input")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetResultsEngagementInput {
    pub value: String,
}

impl protocol::MutationKind<JackConfig, JackConfigMutation> for SetResultsEngagementInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "results-engagement-input", kind: "set-results-engagement-input", record: "SetResultsEngagementInput" };
    fn diff(&self, base: &JackConfig) -> protocol::MutationOutcome<JackConfig> {
        let mut next = base.clone();
        next.results_engagement_input = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &JackConfig) -> Vec<JackConfigMutation> { vec![JackConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Results Engagement Input".into() }
    fn target(&self) -> Vec<String> { vec!["results_engagement_input".into()] }
}
