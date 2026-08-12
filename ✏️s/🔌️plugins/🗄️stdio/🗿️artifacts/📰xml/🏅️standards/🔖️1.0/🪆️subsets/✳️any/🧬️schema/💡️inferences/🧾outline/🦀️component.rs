//! 🧾 `outline` — one named inference: this XML document's own tree shape. `elementCount` is a
//! real recursive walk counting every `XmlNode::Element` (text/CDATA/comment/PI nodes don't
//! count — they carry no further structure); `maxDepth` is the deepest element nesting level (a
//! document with only a root element is depth 1, no root at all is depth 0); `hasDoctype` mirrors
//! the snapshot's own `doctype` presence.

use crate::artifacts::xml::XmlSnapshot;
use crate::artifacts::xml::schema::snapshot::XmlNode;
use serde::{Deserialize, Serialize};

//#region 🔖️Outline
/// 🧾️ `Xml` document outline.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XmlOutline {
    pub element_count: u32,
    pub max_depth: u32,
    pub has_doctype: bool,
}

/// 🌳️ Recursively walks an element subtree, returning `(element_count, max_depth)` — `depth` is
/// the caller's own nesting level (the root element call passes `1`). Non-`Element` nodes
/// (`Text`/`CData`/`Comment`/`ProcessingInstruction`) don't recurse further and don't count.
fn walk(node: &XmlNode, depth: u32) -> (u32, u32) {
    match node {
        XmlNode::Element { children, .. } => {
            let mut count = 1u32;
            let mut max_depth = depth;
            for child in children {
                let (c, d) = walk(child, depth + 1);
                count += c;
                max_depth = max_depth.max(d);
            }
            (count, max_depth)
        }
        _ => (0, depth - 1),
    }
}

impl XmlOutline {
    pub fn compute(snapshot: &XmlSnapshot) -> Self {
        let (element_count, max_depth) = match &snapshot.doc.root {
            Some(root) => walk(root, 1),
            None => (0, 0),
        };
        Self { element_count, max_depth, has_doctype: snapshot.doc.doctype.is_some() }
    }
}
//#endregion 🔖️Outline

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::xml::schema::snapshot::XmlDocument;

    #[test]
    fn counts_elements_and_depth_over_nested_structure() {
        let root = XmlNode::Element {
            name: "root".into(),
            attrs: vec![],
            children: vec![XmlNode::Element { name: "child".into(), attrs: vec![], children: vec![XmlNode::Text { text: "hi".into() }] }],
        };
        let snapshot = XmlSnapshot { schema: "stdio.xml".into(), doc: XmlDocument { root: Some(root), doctype: Some("<!DOCTYPE root>".into()), declaration: None } };
        let outline = XmlOutline::compute(&snapshot);
        assert_eq!(outline.element_count, 2);
        assert_eq!(outline.max_depth, 2);
        assert!(outline.has_doctype);
    }

    #[test]
    fn empty_document_has_zero_elements_and_depth() {
        let outline = XmlOutline::compute(&XmlSnapshot::default());
        assert_eq!(outline.element_count, 0);
        assert_eq!(outline.max_depth, 0);
        assert!(!outline.has_doctype);
    }

    #[test]
    fn outline_is_deterministic() {
        let snapshot = XmlSnapshot::default();
        assert_eq!(XmlOutline::compute(&snapshot), XmlOutline::compute(&snapshot));
    }
}
//#endregion 🧪️Tests
