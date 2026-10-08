//! 🔤️ Sets the selection for one concrete Jack editor window.

use super::{JackEditorWindowTransient, JackEditorWindowTransientDiff, JackEditorWindowTransientMutation};
use crate::editor::jack::transient::JackEditorSelection;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[dsl(keyword = "set-editor-selection")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetEditorSelection {
    #[dsl(block)]
    pub selection: Option<JackEditorSelection>,
}

impl protocol::MutationKind<JackEditorWindowTransient, JackEditorWindowTransientMutation> for SetEditorSelection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "editor-window-selection", kind: "set-editor-selection", record: "SetEditorSelection" };

    fn diff(&self, base: &JackEditorWindowTransient) -> protocol::MutationOutcome<JackEditorWindowTransientDiff> {
        if self.selection == base.selection {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The editor window selection is unchanged.");
        }
        protocol::MutationOutcome::new(JackEditorWindowTransientDiff { selection: Some(self.selection.clone()) })
    }

    fn inverse(&self, base: &JackEditorWindowTransient) -> Result<Vec<JackEditorWindowTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { selection: base.selection.clone() }.into()]
    
    })())
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Editor Window Selection", "Auswahl im Editorfenster setzen")
    }

    fn target(&self) -> Vec<String> {
        vec!["selection".into()]
    }
}
