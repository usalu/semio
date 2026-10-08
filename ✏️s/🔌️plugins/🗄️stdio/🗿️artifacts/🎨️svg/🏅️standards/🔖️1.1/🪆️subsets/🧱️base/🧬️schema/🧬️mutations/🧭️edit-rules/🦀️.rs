//! 🧭️ The details-pane vocabulary of the SVG 1.1 editor: which JSON-pointer edit raises which concrete kind.
//!
//! [`EDIT_RULES`] names the two document fields one kind sets; [`special`] resolves every pointer below `/doc/root` to the kinds of its gesture
//! through [`tree_edit`] — an element is addressed by its node path, an attribute by its name, a child by its position.

use super::*;
use crate::schema::mutation_support::{region_edit, tree_edit, TreeEditKit};
use crate::schema::snapshot::{NodePath, SvgAttributeValue, SvgNode};
use crate::SvgSnapshot;
use semio_s_artifact_stdio_contract::editing::{EditRules, EntityRule, SnapshotEditError, SnapshotEditEvent};

/// 📚 Every document field one SVG kind sets.
pub const EDIT_RULES: EditRules = EditRules {
    entities: &[EntityRule::new("/doc/declaration", "set-declaration", "declaration"), EntityRule::new("/doc/doctype", "set-doctype", "doctype")],
    inserts: &[],
    removes: &[],
};

struct BaseKit;

impl TreeEditKit for BaseKit {
    type Mutation = SvgMutation;

    fn set_attribute(path: NodePath, name: String, value: Option<SvgAttributeValue>, index: Option<usize>) -> SvgMutation {
        SvgMutation::SetAttribute(SetAttributePayload { path, name, value, index })
    }
    fn insert_element(parent: NodePath, index: usize, node: SvgNode) -> SvgMutation {
        SvgMutation::InsertElement(InsertElementPayload { parent, index, node })
    }
    fn remove_element(parent: NodePath, index: usize) -> SvgMutation {
        SvgMutation::RemoveElement(RemoveElementPayload { parent, index })
    }
    fn set_text(path: NodePath, text: String) -> SvgMutation {
        SvgMutation::SetText(SetTextPayload { path, text })
    }
    fn set_element_name(path: NodePath, name: String) -> Option<SvgMutation> {
        Some(SvgMutation::SetElementName(SetElementNamePayload { path, name }))
    }
    fn set_declaration(declaration: Option<semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration>) -> Option<SvgMutation> {
        Some(SvgMutation::SetDeclaration(SetDeclarationPayload { declaration }))
    }
    fn set_doctype(doctype: Option<semio_s_artifact_stdio_xml::schema::snapshot::XmlDoctype>) -> Option<SvgMutation> {
        Some(SvgMutation::SetDoctype(SetDoctypePayload { doctype }))
    }
}

/// 🖼️ The kinds that replace the content of the drawing's root element by the one `region` describes (the `set-pixel-region` gesture).
pub fn region(base: &SvgSnapshot, region: &SvgSnapshot) -> Result<Vec<SvgMutation>, String> {
    region_edit::<BaseKit>(base, region)
}

/// 🎯 The kinds of an edit below `/doc/root`; `None` hands the edit on to the table.
pub fn special(event: &SnapshotEditEvent, snapshot: &SvgSnapshot) -> Result<Option<Vec<SvgMutation>>, SnapshotEditError> {
    tree_edit::<BaseKit>(event, snapshot)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
