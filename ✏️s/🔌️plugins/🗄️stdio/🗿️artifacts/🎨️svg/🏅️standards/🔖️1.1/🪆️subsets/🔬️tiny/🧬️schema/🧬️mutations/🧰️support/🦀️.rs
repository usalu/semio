//! 🧰️ `SvgTinyMutation` support: the diff builders and base accessors its mutation leaves call, hosted below the leaves so that no leaf has to import the aggregate module.

use crate::schema::snapshot::SvgNode;

/// 🚫 Elements SVG Tiny 1.1 excludes outright; `fe*` filter primitives match by prefix, since Tiny
/// forbids the whole filter mechanism.
pub const BLOCKED_ELEMENTS: &[&str] = &["style", "script", "symbol", "marker", "clipPath", "mask", "pattern", "linearGradient", "radialGradient", "stop", "filter", "cursor", "textPath", "tspan", "tref", "view"];

/// 🚫 Presentation attributes SVG Tiny 1.1 forbids on ANY element.
pub const BLOCKED_ATTRS: &[&str] = &["style", "opacity", "fill-opacity", "stroke-opacity", "clip-path", "mask", "filter"];

pub const CODE_REJECTED: &str = "mutation.target-mismatch";

pub fn local_name(name: &str) -> &str {
    name.rsplit(':').next().unwrap_or(name)
}

/// 🚫 `true` for an element SVG Tiny 1.1 does not retain.
pub fn is_blocked_element(name: &str) -> bool {
    let ln = local_name(name);
    BLOCKED_ELEMENTS.contains(&ln) || ln.starts_with("fe")
}

/// 🚫 `true` for a presentation attribute SVG Tiny 1.1 forbids everywhere.
pub fn is_blocked_attribute(name: &str) -> bool {
    BLOCKED_ATTRS.contains(&local_name(name))
}

/// 🛡️ The gate every authoring mutation passes through: the message naming the first excluded
/// element or attribute in the subtree, or `None` when the subtree is Tiny-clean.
pub fn subtree_profile_violation(node: &SvgNode) -> Option<String> {
    match node {
        SvgNode::Element { name, attrs, children } => {
            if is_blocked_element(name) {
                return Some(format!("element <{name}> is outside SVG Tiny 1.1's vocabulary -- REC-SVGMobile-20030114 excludes it"));
            }
            if let Some(attr) = attrs.iter().find(|a| is_blocked_attribute(&a.name)) {
                return Some(format!("attribute '{}' on <{name}> is forbidden anywhere in SVG Tiny 1.1", attr.name));
            }
            children.iter().find_map(subtree_profile_violation)
        }
        _ => None,
    }
}
