//! ♻️ `restore-non-tiny` — puts back the excluded elements and attributes a `strip-non-tiny` removed, at their exact positions. It is the
//! inverse of `strip-non-tiny` and deliberately skips the profile gate the authoring leaves apply, because it only ever restores content
//! the document held before.

use super::*;
use crate::schema::diff::{SvgAttrAdded, SvgAttributesDiff, SvgChildModified};
use crate::schema::snapshot::SvgNode as Node;

//#region 🔖️Payload
/// 🧩 One excluded child element and where it sits among its parent's children once restored.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RestoredElement {
    pub parent: NodePath,
    pub index: usize,
    pub node: SvgNode,
}

/// 🏷️ One excluded attribute and the position it takes among its element's attributes once restored.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct RestoredAttribute {
    pub path: NodePath,
    pub index: usize,
    pub name: String,
    pub value: SvgAttributeValue,
}

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RestoreNonTiny {
    #[value(default)]
    pub elements: Vec<RestoredElement>,
    #[value(default)]
    pub attributes: Vec<RestoredAttribute>,
}

/// 🔺️ The node diff that adds the rows addressed at or below `node` (which sits at `path`), or `None` when none are.
fn restore_node_diff(node: &Node, path: &NodePath, payload: &RestoreNonTiny) -> Option<SvgNodeDiff> {
    let Node::Element { children, .. } = node else { return None };
    let mut attributes: Vec<SvgAttrAdded> = payload.attributes.iter().filter(|row| row.path == *path).map(|row| SvgAttrAdded { index: row.index, name: row.name.clone(), value: row.value.clone() }).collect();
    attributes.sort_by_key(|row| row.index);
    let mut added: Vec<SvgChildAdded> = payload.elements.iter().filter(|row| row.parent == *path).map(|row| SvgChildAdded { index: row.index, item: row.node.clone() }).collect();
    added.sort_by_key(|row| row.index);
    let modified: Vec<SvgChildModified> = children
        .iter()
        .enumerate()
        .filter_map(|(index, child)| restore_node_diff(child, &[path.as_slice(), &[index]].concat(), payload).map(|diff| SvgChildModified { index, diff }))
        .collect();
    if attributes.is_empty() && added.is_empty() && modified.is_empty() {
        return None;
    }
    Some(SvgNodeDiff::Element(SvgElementDiff {
        name: None,
        attributes: (!attributes.is_empty()).then(|| SvgAttributesDiff { added: attributes, ..Default::default() }),
        children: (!added.is_empty() || !modified.is_empty()).then(|| SvgChildrenDiff { modified, added, ..Default::default() }),
    }))
}

impl protocol::MutationKind<SvgSnapshot, SvgTinyMutation> for RestoreNonTiny {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "non-tiny-content", kind: "restore-non-tiny", record: "RestoredNonTinyContent" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let addressed = |path: &NodePath| matches!(node_at(&base.doc, path), Ok(Node::Element { .. }));
        if !self.elements.iter().all(|row| addressed(&row.parent)) || !self.attributes.iter().all(|row| addressed(&row.path)) {
            return protocol::MutationOutcome::error(CODE_REJECTED, "a restored row addresses a node that is not an element of this document".to_string(), Vec::<String>::new());
        }
        protocol::MutationOutcome::new(SvgDiff { root: base.doc.root.as_ref().and_then(|root| restore_node_diff(root, &Vec::new(), self)), ..Default::default() })
    }
    fn inverse(&self, _base: &SvgSnapshot) -> Result<Vec<SvgTinyMutation>, semio_framework_value::ValueError> {
        Ok(if self.elements.is_empty() && self.attributes.is_empty() { Vec::new() } else { vec![SvgTinyMutation::StripNonTiny(strip_non_tiny::StripNonTiny {})] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Restore non-Tiny content", "Nicht-Tiny-Inhalte wiederherstellen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
