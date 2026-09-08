//! 🔤️ Sets the selection for one concrete Jack editor window.

use super::{JackEditorSelection, JackTransient, JackTransientMutation};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-editor-selection")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEditorSelection {
    pub window_id: String,
    #[dsl(block)]
    pub selection: Option<JackEditorSelection>,
}

impl protocol::MutationKind<JackTransient, JackTransientMutation> for SetEditorSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "editor-window-selection", kind: "set-editor-selection", record: "SetEditorSelection" };

    fn diff(&self, base: &JackTransient) -> protocol::MutationOutcome<JackTransient> {
        let mut next = base.clone();
        match &self.selection {
            Some(selection) => {
                next.editor_selection_by_window_id.insert(self.window_id.clone(), selection.clone());
            }
            None => {
                next.editor_selection_by_window_id.remove(&self.window_id);
            }
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &JackTransient) -> Vec<JackTransientMutation> {
        vec![Self { window_id: self.window_id.clone(), selection: base.editor_selection_by_window_id.get(&self.window_id).cloned() }.into()]
    }

    fn label(&self) -> String {
        "Set Editor Window Selection".into()
    }

    fn target(&self) -> Vec<String> {
        vec![format!("editor_selection_by_window_id.{}", self.window_id)]
    }
}
