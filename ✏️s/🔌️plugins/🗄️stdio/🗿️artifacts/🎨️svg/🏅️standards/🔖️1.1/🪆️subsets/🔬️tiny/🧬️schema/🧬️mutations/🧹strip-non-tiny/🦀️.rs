//! 🧹️ `strip-non-tiny` — drops every excluded element subtree and forbidden presentation attribute of the document. Its diff names
//! exactly the removed attributes and children; its inverse is the `restore-non-tiny` leaf carrying the removed rows.
//!
//! `strip` is not itself an approved semantic verb (`protocol::APPROVED_VERBS`); this leaf performs
//! the Full→Tiny down-conversion by DROPPING every excluded element subtree and forbidden
//! presentation attribute, so `SEMANTICS.verb` is `"remove"`.

use super::restore_non_tiny::{RestoredAttribute, RestoredElement};
use super::*;
use crate::schema::diff::{SvgAttributesDiff, SvgChildModified};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct StripNonTiny {}

/// 🔺️ The node diff that removes the excluded attributes and children below `node`, or `None` when it holds none.
fn strip_node_diff(node: &SvgNode) -> Option<SvgNodeDiff> {
    let SvgNode::Element { attrs, children, .. } = node else { return None };
    let removed_attributes: Vec<String> = attrs.iter().filter(|attribute| is_blocked_attribute(&attribute.name)).map(|attribute| attribute.name.clone()).collect();
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for (index, child) in children.iter().enumerate() {
        match child {
            SvgNode::Element { name, .. } if is_blocked_element(name) => removed.push(index),
            _ => modified.extend(strip_node_diff(child).map(|diff| SvgChildModified { index, diff })),
        }
    }
    if removed_attributes.is_empty() && removed.is_empty() && modified.is_empty() {
        return None;
    }
    Some(SvgNodeDiff::Element(SvgElementDiff {
        name: None,
        attributes: (!removed_attributes.is_empty()).then(|| SvgAttributesDiff { removed: removed_attributes, ..Default::default() }),
        children: (!removed.is_empty() || !modified.is_empty()).then(|| SvgChildrenDiff { removed, modified, ..Default::default() }),
    }))
}

/// ♻️ The rows that put back what stripping removes below `node`, addressed in the stripped tree.
fn stripped_rows(node: &SvgNode, path: &mut NodePath, elements: &mut Vec<RestoredElement>, attributes: &mut Vec<RestoredAttribute>) {
    let SvgNode::Element { attrs, children, .. } = node else { return };
    attributes.extend(attrs.iter().enumerate().filter(|(_, attribute)| is_blocked_attribute(&attribute.name)).map(|(index, attribute)| RestoredAttribute { path: path.clone(), index, name: attribute.name.clone(), value: attribute.value.clone() }));
    let mut kept = 0;
    for (index, child) in children.iter().enumerate() {
        match child {
            SvgNode::Element { name, .. } if is_blocked_element(name) => elements.push(RestoredElement { parent: path.clone(), index, node: child.clone() }),
            _ => {
                path.push(kept);
                stripped_rows(child, path, elements, attributes);
                path.pop();
                kept += 1;
            }
        }
    }
}

impl protocol::MutationKind<SvgSnapshot, SvgTinyMutation> for StripNonTiny {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "non-tiny-content", kind: "strip-non-tiny", record: "StripNonTiny" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        protocol::MutationOutcome::new(SvgDiff { root: base.doc.root.as_ref().and_then(strip_node_diff), ..Default::default() })
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgTinyMutation>, semio_framework_value::ValueError> {
        let (mut elements, mut attributes) = (Vec::new(), Vec::new());
        if let Some(root) = &base.doc.root {
            stripped_rows(root, &mut Vec::new(), &mut elements, &mut attributes);
        }
        Ok(if elements.is_empty() && attributes.is_empty() { Vec::new() } else { vec![SvgTinyMutation::RestoreNonTiny(restore_non_tiny::RestoreNonTiny { elements, attributes })] })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Strip non-Tiny content", "Nicht-Tiny-Inhalte entfernen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
