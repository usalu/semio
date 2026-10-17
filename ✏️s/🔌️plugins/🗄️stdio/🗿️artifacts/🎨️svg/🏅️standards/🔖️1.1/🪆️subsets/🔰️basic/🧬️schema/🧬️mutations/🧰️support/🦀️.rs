//! 🧰️ `SvgBasicMutation` support: the diff builders and base accessors its mutation leaves call, hosted below the leaves so that no leaf has to import the aggregate module.

use crate::schema::snapshot::NodePath;
use crate::schema::diff::SvgChildAdded;
use crate::schema::diff::SvgChildrenDiff;
use crate::schema::diff::SvgDiff;
use crate::schema::diff::SvgElementDiff;
use crate::schema::snapshot::SvgNode;
use crate::schema::diff::SvgNodeDiff;
use crate::SvgSnapshot;
use crate::schema::diff::diff_at_path;
use crate::schema::snapshot::node_at;

/// 🚫 The expensive raster filter primitives SVG Basic 1.1 excludes.
pub const BLOCKED_FILTER_PRIMITIVES: &[&str] = &["feConvolveMatrix", "feDisplacementMap", "feTurbulence", "feMorphology", "feDiffuseLighting", "feSpecularLighting", "feDistantLight", "fePointLight", "feSpotLight"];

/// ✍️ The SVG text element kinds — a clip path containing one clips to text.
pub const TEXT_ELEMENTS: &[&str] = &["text", "tspan", "tref", "textPath"];

pub const CODE_REJECTED: &str = "mutation.target-mismatch";

pub fn local_name(name: &str) -> &str {
    name.rsplit(':').next().unwrap_or(name)
}

/// 🚫 `true` for a raster filter primitive SVG Basic 1.1 does not retain.
pub fn is_blocked_filter_primitive(name: &str) -> bool {
    BLOCKED_FILTER_PRIMITIVES.contains(&local_name(name))
}

/// ✍️ `true` when `node` is, or contains, an SVG text element.
pub fn carries_text(node: &SvgNode) -> bool {
    match node {
        SvgNode::Element { name, children, .. } => TEXT_ELEMENTS.contains(&local_name(name)) || children.iter().any(carries_text),
        _ => false,
    }
}

/// 🛡️ The subtree gate: the message naming the first excluded primitive, or the clip-to-text
/// violation, or `None` when the subtree is Basic-clean.
pub fn subtree_profile_violation(node: &SvgNode) -> Option<String> {
    match node {
        SvgNode::Element { name, children, .. } => {
            if is_blocked_filter_primitive(name) {
                return Some(format!("element <{name}> is an expensive raster filter primitive not supported by SVG Basic 1.1"));
            }
            if local_name(name) == "clipPath" && children.iter().any(carries_text) {
                return Some(format!("<{name}> contains a text descendant -- SVG Basic 1.1 forbids clipping to text"));
            }
            children.iter().find_map(subtree_profile_violation)
        }
        _ => None,
    }
}

/// 🗺️ The `NodePath` of the first element carrying `id`, depth-first from the root.
pub fn path_of_id(snapshot: &SvgSnapshot, id: &str) -> Option<NodePath> {
    fn walk(node: &SvgNode, id: &str, prefix: &mut NodePath) -> Option<NodePath> {
        if let SvgNode::Element { attrs, children, .. } = node {
            if attrs.iter().any(|a| a.name == "id" && a.value.text() == Some(id)) {
                return Some(prefix.clone());
            }
            for (index, child) in children.iter().enumerate() {
                prefix.push(index);
                if let Some(found) = walk(child, id, prefix) {
                    return Some(found);
                }
                prefix.pop();
            }
        }
        None
    }
    walk(snapshot.doc.root.as_ref()?, id, &mut Vec::new())
}

/// 🛡️ Resolves `id` to a clip path this profile allows a reference to point at, or the message
/// saying why it does not.
pub fn resolve_clip_path(snapshot: &SvgSnapshot, id: &str) -> Result<NodePath, String> {
    let path = path_of_id(snapshot, id).ok_or_else(|| format!("this document declares no element with id {id:?}"))?;
    match node_at(&snapshot.doc, &path) {
        Ok(SvgNode::Element { name, children, .. }) if local_name(name) == "clipPath" => {
            if children.iter().any(carries_text) {
                return Err(format!("clipPath #{id} contains a text descendant -- SVG Basic 1.1 forbids clipping to text"));
            }
            Ok(path)
        }
        _ => Err(format!("#{id} is not a clipPath element")),
    }
}

pub fn insert_child_diff(parent: &[usize], index: usize, node: &SvgNode) -> SvgDiff {
    diff_at_path(parent, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgChildAdded { index, item: node.clone() }] }) }))
}

pub fn remove_child_diff(parent: &[usize], index: usize) -> SvgDiff {
    diff_at_path(parent, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: vec![index], modified: Vec::new(), added: Vec::new() }) }))
}
