//! 🔤️ Set Editor Selection in the Trinity jack configuration channel.

use super::{JackConfig, JackConfigMutation, ReplaceConfig, JackEditorSelection};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-editor-selection")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEditorSelection {
    #[dsl(block)]
    pub selection: Option<JackEditorSelection>,
}

impl protocol::MutationKind<JackConfig, JackConfigMutation> for SetEditorSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "editor-selection", kind: "set-editor-selection", record: "SetEditorSelection" };
    fn diff(&self, base: &JackConfig) -> protocol::MutationOutcome<JackConfig> {
        let mut next = base.clone();
        next.editor_selection = self.selection.clone();
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &JackConfig) -> Vec<JackConfigMutation> { vec![JackConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })] }
    fn label(&self) -> String { "Set Editor Selection".into() }
    fn target(&self) -> Vec<String> { vec!["editor_selection".into()] }
}
