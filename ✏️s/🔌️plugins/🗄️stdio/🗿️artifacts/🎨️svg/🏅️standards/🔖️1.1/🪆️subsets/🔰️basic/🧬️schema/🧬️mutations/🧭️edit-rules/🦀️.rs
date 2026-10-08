//! 🧭️ The details-pane vocabulary of the SVG 1.1 basic editor: which JSON-pointer edit raises which concrete kind.
//!
//! The subset has no document-field kind, so [`EDIT_RULES`] is empty; [`special`] resolves every pointer below `/doc/root` to the kinds of its
//! gesture through [`tree_edit`] — an element is addressed by its node path, an attribute by its name, a child by its position.

use super::*;
use crate::schema::mutation_support::{region_edit, tree_edit, TreeEditKit};
use crate::schema::snapshot::{NodePath, SvgAttributeValue, SvgNode};
use semio_s_artifact_stdio_contract::editing::{EditRules, SnapshotEditError, SnapshotEditEvent};

/// 📚 The subset declares no pointer outside the element tree.
pub const EDIT_RULES: EditRules = EditRules { entities: &[], inserts: &[], removes: &[] };

struct Kit;

impl TreeEditKit for Kit {
    type Mutation = SvgBasicMutation;

    fn set_attribute(path: NodePath, name: String, value: Option<SvgAttributeValue>, index: Option<usize>) -> SvgBasicMutation {
        SvgBasicMutation::SetBasicAttribute(set_basic_attribute::SetBasicAttribute { path, name, value, index })
    }
    fn insert_element(parent: NodePath, index: usize, node: SvgNode) -> SvgBasicMutation {
        SvgBasicMutation::InsertBasicElement(insert_basic_element::InsertBasicElement { parent, index, node })
    }
    fn remove_element(parent: NodePath, index: usize) -> SvgBasicMutation {
        SvgBasicMutation::RemoveElement(remove_element::RemoveElement { parent, index })
    }
    fn set_text(path: NodePath, text: String) -> SvgBasicMutation {
        SvgBasicMutation::SetText(set_text::SetText { path, text })
    }
}

/// 🖼️ The kinds that replace the content of the drawing's root element by the one `region` describes (the `set-pixel-region` gesture).
pub fn region(base: &SvgSnapshot, region: &SvgSnapshot) -> Result<Vec<SvgBasicMutation>, String> {
    region_edit::<Kit>(base, region)
}

/// 🎯 The kinds of an edit below `/doc/root`; `None` hands the edit on to the (empty) table.
pub fn special(event: &SnapshotEditEvent, snapshot: &SvgSnapshot) -> Result<Option<Vec<SvgBasicMutation>>, SnapshotEditError> {
    tree_edit::<Kit>(event, snapshot)
}
