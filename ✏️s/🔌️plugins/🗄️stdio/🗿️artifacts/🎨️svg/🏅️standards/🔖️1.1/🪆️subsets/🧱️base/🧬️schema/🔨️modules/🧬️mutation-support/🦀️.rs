//! 🧰 Shared attribute diff construction for direct SVG mutations.
use crate::schema::diff::{diff_at_path, SvgAttrAdded, SvgAttrModified, SvgAttributesDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::schema::snapshot::node_at;
use crate::SvgSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument, XmlNode};

pub fn attribute_diff_at_path(base: &SvgSnapshot, path: &[usize], name: &str, value: Option<String>) -> SvgDiff {
    let target = node_at(&base.doc, path).ok();
    let existing = target.and_then(|node| match node {
        XmlNode::Element { attrs, .. } => attrs.iter().find(|attribute| attribute.name == name),
        _ => None,
    });
    let attributes = match (existing, value) {
        (Some(_), Some(value)) => SvgAttributesDiff { removed: Vec::new(), modified: vec![SvgAttrModified { name: name.to_string(), value }], added: Vec::new() },
        (Some(_), None) => SvgAttributesDiff { removed: vec![name.to_string()], modified: Vec::new(), added: Vec::new() },
        (None, Some(value)) => {
            let index = match target {
                Some(XmlNode::Element { attrs, .. }) => attrs.len(),
                _ => 0,
            };
            SvgAttributesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgAttrAdded { index, name: name.to_string(), value }] }
        }
        (None, None) => SvgAttributesDiff::default(),
    };
    diff_at_path(path, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: Some(attributes), children: None }))
}








