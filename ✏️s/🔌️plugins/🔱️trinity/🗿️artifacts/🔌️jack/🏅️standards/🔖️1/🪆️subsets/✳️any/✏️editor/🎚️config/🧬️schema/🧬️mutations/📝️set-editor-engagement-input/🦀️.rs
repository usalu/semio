//! 📝️ Set Editor Engagement Input in the Trinity jack configuration channel.

use super::{JackConfig, JackConfigMutation, ReplaceConfig};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-editor-engagement-input")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEditorEngagementInput {
    pub value: String,
}

impl protocol::MutationKind<JackConfig, JackConfigMutation> for SetEditorEngagementInput {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "editor-engagement-input", kind: "set-editor-engagement-input", record: "SetEditorEngagementInput" };
    fn diff(&self, base: &JackConfig) -> protocol::MutationOutcome<JackConfig> {
        let mut next = base.clone();
        next.editor_engagement_input = self.value.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &JackConfig) -> Vec<JackConfigMutation> { vec![JackConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Editor Engagement Input".into() }
    fn target(&self) -> Vec<String> { vec!["editor_engagement_input".into()] }
}
