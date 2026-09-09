//! 🔎️ Sets the authored query of the addressed Jack editor window.

use super::{JackEditorWindowConfig, JackEditorWindowConfigMutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-query")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetQuery {
    pub value: String,
}

impl protocol::MutationKind<JackEditorWindowConfig, JackEditorWindowConfigMutation> for SetQuery {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "editor-query", kind: "set-query", record: "SetQuery" };
    fn diff(&self, base: &JackEditorWindowConfig) -> protocol::MutationOutcome<JackEditorWindowConfig> {
        let mut next = base.clone();
        next.jack_query.clone_from(&self.value);
        protocol::MutationOutcome::new(next)
    }
    fn inverse(&self, base: &JackEditorWindowConfig) -> Vec<JackEditorWindowConfigMutation> {
        vec![Self { value: base.jack_query.clone() }.into()]
    }
    fn label(&self) -> String { "Set Editor Query".into() }
    fn target(&self) -> Vec<String> { vec!["jack_query".into()] }
}
