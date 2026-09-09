//! 🔤️ Sets the selection for one concrete Jack editor window.

use super::{JackEditorWindowTransient, JackEditorWindowTransientMutation};
use crate::editor::jack::transient::JackEditorSelection;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-editor-selection")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEditorSelection {
    #[dsl(block)]
    pub selection: Option<JackEditorSelection>,
}

impl protocol::MutationKind<JackEditorWindowTransient, JackEditorWindowTransientMutation> for SetEditorSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "editor-window-selection", kind: "set-editor-selection", record: "SetEditorSelection" };

    fn diff(&self, base: &JackEditorWindowTransient) -> protocol::MutationOutcome<JackEditorWindowTransient> {
        let mut next = base.clone();
        next.selection.clone_from(&self.selection);
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &JackEditorWindowTransient) -> Vec<JackEditorWindowTransientMutation> {
        vec![Self { selection: base.selection.clone() }.into()]
    }

    fn label(&self) -> String {
        "Set Editor Window Selection".into()
    }

    fn target(&self) -> Vec<String> {
        vec!["selection".into()]
    }
}
