//! 🧬️ `SvgBasicMutation` — the SVG Basic 1.1 mutation vocabulary. Handcrafted for THIS subset, not
//! inherited from `✳️any` and not a longer allow-list of `✳️tiny`'s.
//!
//! SVG Basic 1.1 (W3C Mobile SVG Profiles, REC-SVGMobile-20030114 §SVG Basic 1.1) KEEPS what Tiny
//! drops — gradients, patterns, masks, opacity, the `clipPath` element and the filter mechanism.
//! What it excludes is narrow and specific: nine expensive raster filter primitives, and clipping to
//! text. Those two exclusions are what this vocabulary is built around.
//! [`SvgBasicMutation::InsertBasicElement`] refuses a subtree carrying an excluded primitive;
//! [`SvgBasicMutation::SetClipPathReference`] and [`SvgBasicMutation::InsertClipPathShape`] address
//! clip paths as first-class subjects and refuse anything that would clip to text. None of the three
//! has a counterpart in `✳️tiny`, whose profile has neither filters nor `clipPath` at all; and
//! `✳️any`'s ungated `SetAttribute`/`InsertElement` can leave the profile in one step.
//!
//! The excluded-vocabulary lists are restated here, beside the vocabulary they gate, because the
//! subset's own `check_svg_basic_conformance` answers a different question — it judges a whole
//! decoded document, while a mutation gate has to judge a candidate SUBTREE and a candidate
//! reference before either enters one.
//! `blocklists_agree_with_the_subset_conformance_checker` below holds the two against each other so
//! they cannot drift apart silently.
//!
//! @see ../../🧪️oracle/🔣️.json — the catalog `KINDS` below must match exactly.
//! @see ../../../../../../🧪️tests/mutate-svg-1-1-basic/🥒️.feature — the case that exercises it.

use crate::artifacts::svg::schema::diff::{diff_at_path, diff_set_snapshot, SvgAttrAdded, SvgAttrModified, SvgAttributesDiff, SvgChildAdded, SvgChildrenDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::artifacts::svg::schema::snapshot::{element_attr, node_at, parse_transform_list, parse_view_box, transform_list_to_string, view_box_to_string, NodePath, TransformOp, ViewBox};
use crate::artifacts::svg::SvgSnapshot;
use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlNode};
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.svg` 1.1/✳️basic. Nodes are addressed by `NodePath`; clip
/// paths are addressed by their `id`, because that is how a `clip-path="url(#id)"` reference names
/// them and the profile's whole clip-path rule is about what a reference resolves to.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SvgBasicMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: SvgSnapshot,
    },
    /// 🏷️ Sets (or, with `None`, clears) the root's `baseProfile`/`version` declaration.
    StampBaseProfile {
        base_profile: Option<String>,
        version: Option<String>,
    },
    /// ➕️ Inserts `node` as child `index` of the element at `parent`, REJECTED when the subtree
    /// carries one of the raster filter primitives SVG Basic 1.1 excludes, or is a clip path that
    /// clips to text.
    InsertBasicElement {
        parent: NodePath,
        index: usize,
        node: XmlNode,
    },
    /// ➖️ Removes child `index` of the element at `parent`.
    RemoveElement {
        parent: NodePath,
        index: usize,
    },
    /// 🏷️ Sets (or, with `value: None`, removes) attribute `name` on the element at `path`,
    /// REJECTED when it is a `clip-path` whose `url(#id)` resolves to a clip path containing text.
    SetBasicAttribute {
        path: NodePath,
        name: String,
        value: Option<String>,
    },
    /// ✂️ Points the element at `path` at the clip path named by `clip_path_id` (or clears its
    /// `clip-path` with `None`). REJECTED when the named clip path does not exist, is not a
    /// `clipPath`, or contains a text descendant — SVG Basic 1.1 does not support clipping to text.
    SetClipPathReference {
        path: NodePath,
        clip_path_id: Option<String>,
    },
    /// ➕️ Adds a clipping shape as child `index` of the clip path named by `clip_path_id`.
    /// REJECTED for a shape that is or contains a text element, for the same profile reason.
    InsertClipPathShape {
        clip_path_id: String,
        index: usize,
        node: XmlNode,
    },
    /// ✍️ Replaces the literal text of the `Text` node at `path`.
    SetText {
        path: NodePath,
        text: String,
    },
    /// 🖼️ Sets (or clears) the typed `viewBox` of the element at `path`.
    SetViewBox {
        path: NodePath,
        view_box: Option<ViewBox>,
    },
    /// 🔄 Sets (or clears) the typed `transform` list of the element at `path`.
    SetTransform {
        path: NodePath,
        transform: Option<Vec<TransformOp>>,
    },
}

/// 📇️ Kebab-case spelling of every `SvgBasicMutation` variant, in declaration order — the exact
/// `kinds` list `../../🧪️oracle/🔣️.json`'s `mutationCatalogs` entry declares.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "stamp-base-profile", "insert-basic-element", "remove-element", "set-basic-attribute", "set-clip-path-reference", "insert-clip-path-shape", "set-text", "set-view-box", "set-transform"];

crate::impl_serde_op_codec!(SvgBasicMutation, "svg-basic-mutation");

/// 🏷️ The `KINDS` spelling of one mutation's own variant, exhaustively matched.
pub fn kind_of(mutation: &SvgBasicMutation) -> &'static str {
    match mutation {
        SvgBasicMutation::NoMutation => "no-mutation",
        SvgBasicMutation::SetSnapshot { .. } => "set-snapshot",
        SvgBasicMutation::StampBaseProfile { .. } => "stamp-base-profile",
        SvgBasicMutation::InsertBasicElement { .. } => "insert-basic-element",
        SvgBasicMutation::RemoveElement { .. } => "remove-element",
        SvgBasicMutation::SetBasicAttribute { .. } => "set-basic-attribute",
        SvgBasicMutation::SetClipPathReference { .. } => "set-clip-path-reference",
        SvgBasicMutation::InsertClipPathShape { .. } => "insert-clip-path-shape",
        SvgBasicMutation::SetText { .. } => "set-text",
        SvgBasicMutation::SetViewBox { .. } => "set-view-box",
        SvgBasicMutation::SetTransform { .. } => "set-transform",
    }
}
//#endregion 🔖️Mutations

//#region 🔖️Profile
/// 🚫 The expensive raster filter primitives SVG Basic 1.1 excludes.
const BLOCKED_FILTER_PRIMITIVES: &[&str] = &["feConvolveMatrix", "feDisplacementMap", "feTurbulence", "feMorphology", "feDiffuseLighting", "feSpecularLighting", "feDistantLight", "fePointLight", "feSpotLight"];

/// ✍️ The SVG text element kinds — a clip path containing one clips to text.
const TEXT_ELEMENTS: &[&str] = &["text", "tspan", "tref", "textPath"];

const CODE_REJECTED: &str = "stdio.svg.basic.mutation-outside-profile";

fn local_name(name: &str) -> &str {
    name.rsplit(':').next().unwrap_or(name)
}

/// 🚫 `true` for a raster filter primitive SVG Basic 1.1 does not retain.
pub fn is_blocked_filter_primitive(name: &str) -> bool {
    BLOCKED_FILTER_PRIMITIVES.contains(&local_name(name))
}

/// ✍️ `true` when `node` is, or contains, an SVG text element.
pub fn carries_text(node: &XmlNode) -> bool {
    match node {
        XmlNode::Element { name, children, .. } => TEXT_ELEMENTS.contains(&local_name(name)) || children.iter().any(carries_text),
        _ => false,
    }
}

/// 🔗 The fragment id of a `clip-path="url(#id)"`-shaped value, bare or quoted.
pub fn clip_path_ref_id(value: &str) -> Option<&str> {
    let inner = value.trim().strip_prefix("url(")?.strip_suffix(')')?;
    inner.trim().trim_matches(|c| c == '\'' || c == '"').strip_prefix('#')
}

/// 🛡️ The subtree gate: the message naming the first excluded primitive, or the clip-to-text
/// violation, or `None` when the subtree is Basic-clean.
pub fn subtree_profile_violation(node: &XmlNode) -> Option<String> {
    match node {
        XmlNode::Element { name, children, .. } => {
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
    fn walk(node: &XmlNode, id: &str, prefix: &mut NodePath) -> Option<NodePath> {
        if let XmlNode::Element { attrs, children, .. } = node {
            if attrs.iter().any(|a| a.name == "id" && a.value == id) {
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
        Ok(XmlNode::Element { name, children, .. }) if local_name(name) == "clipPath" => {
            if children.iter().any(carries_text) {
                return Err(format!("clipPath #{id} contains a text descendant -- SVG Basic 1.1 forbids clipping to text"));
            }
            Ok(path)
        }
        _ => Err(format!("#{id} is not a clipPath element")),
    }
}
//#endregion 🔖️Profile

//#region 🔖️AttributeHelper
/// 🏷️ Builds the exact `SvgAttributesDiff` a set of `(name, value)` transitions requires on the
/// element addressed by `path`, resolving each attribute's PRIOR presence against `base`.
fn attributes_diff_at_path(base: &SvgSnapshot, path: &[usize], changes: &[(&str, Option<String>)]) -> SvgDiff {
    let target = node_at(&base.doc, path).ok();
    let existing: &[XmlAttr] = match target {
        Some(XmlNode::Element { attrs, .. }) => attrs.as_slice(),
        _ => &[],
    };
    let mut diff = SvgAttributesDiff::default();
    let mut next_index = existing.len();
    for (name, value) in changes {
        let present = existing.iter().any(|a| a.name.as_str() == *name);
        match (present, value) {
            (true, Some(v)) => diff.modified.push(SvgAttrModified { name: (*name).to_string(), value: v.clone() }),
            (true, None) => {
                diff.removed.push((*name).to_string());
                next_index -= 1;
            }
            (false, Some(v)) => {
                diff.added.push(SvgAttrAdded { index: next_index, name: (*name).to_string(), value: v.clone() });
                next_index += 1;
            }
            (false, None) => {}
        }
    }
    diff_at_path(path, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: Some(diff), children: None }))
}

fn prior_attribute(base: &SvgSnapshot, path: &[usize], name: &str) -> Option<String> {
    node_at(&base.doc, path).ok().and_then(|n| element_attr(n, name)).map(|s| s.to_string())
}

fn insert_child_diff(parent: &[usize], index: usize, node: &XmlNode) -> SvgDiff {
    diff_at_path(
        parent,
        SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgChildAdded { index, item: node.clone() }] }) }),
    )
}

fn remove_child_diff(parent: &[usize], index: usize) -> SvgDiff {
    diff_at_path(parent, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: vec![index], modified: Vec::new(), added: Vec::new() }) }))
}
//#endregion 🔖️AttributeHelper

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: the diff is the single semantics source.
pub fn apply_svg_basic_mutation(snapshot: &mut SvgSnapshot, mutation: &SvgBasicMutation) -> protocol::MutationOutcome<SvgDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

/// ↩️ This subset's own inverse algebra as a free function, so a caller that legitimately drives the
/// vocabulary from outside the crate can reach it without naming the `protocol::Mutation` trait.
pub fn inverse_svg_basic_mutation(mutation: &SvgBasicMutation, base: &SvgSnapshot) -> Vec<SvgBasicMutation> {
    Mutation::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<SvgSnapshot> for SvgBasicMutation {
    type Diff = SvgDiff;

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            SvgBasicMutation::NoMutation => protocol::MutationOutcome::new(SvgDiff::default()),
            SvgBasicMutation::SetSnapshot { snapshot } => protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot)),
            SvgBasicMutation::StampBaseProfile { base_profile, version } => protocol::MutationOutcome::new(attributes_diff_at_path(base, &[], &[("baseProfile", base_profile.clone()), ("version", version.clone())])),
            SvgBasicMutation::InsertBasicElement { parent, index, node } => match subtree_profile_violation(node) {
                Some(message) => protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new()),
                None => protocol::MutationOutcome::new(insert_child_diff(parent, *index, node)),
            },
            SvgBasicMutation::RemoveElement { parent, index } => protocol::MutationOutcome::new(remove_child_diff(parent, *index)),
            SvgBasicMutation::SetBasicAttribute { path, name, value } => {
                if local_name(name) == "clip-path" {
                    if let Some(id) = value.as_deref().and_then(clip_path_ref_id) {
                        if let Err(message) = resolve_clip_path(base, id) {
                            return protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new());
                        }
                    }
                }
                protocol::MutationOutcome::new(attributes_diff_at_path(base, path, &[(name.as_str(), value.clone())]))
            }
            SvgBasicMutation::SetClipPathReference { path, clip_path_id } => {
                let value = match clip_path_id {
                    Some(id) => match resolve_clip_path(base, id) {
                        Ok(_) => Some(format!("url(#{id})")),
                        Err(message) => return protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new()),
                    },
                    None => None,
                };
                protocol::MutationOutcome::new(attributes_diff_at_path(base, path, &[("clip-path", value)]))
            }
            SvgBasicMutation::InsertClipPathShape { clip_path_id, index, node } => {
                if carries_text(node) {
                    return protocol::MutationOutcome::error(CODE_REJECTED, "the inserted shape carries a text element -- SVG Basic 1.1 forbids clipping to text".to_string(), Vec::<String>::new());
                }
                if let Some(message) = subtree_profile_violation(node) {
                    return protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new());
                }
                match resolve_clip_path(base, clip_path_id) {
                    Ok(target) => protocol::MutationOutcome::new(insert_child_diff(&target, *index, node)),
                    Err(message) => protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new()),
                }
            }
            SvgBasicMutation::SetText { path, text } => protocol::MutationOutcome::new(diff_at_path(path, SvgNodeDiff::Text { text: Some(text.clone()) })),
            SvgBasicMutation::SetViewBox { path, view_box } => protocol::MutationOutcome::new(attributes_diff_at_path(base, path, &[("viewBox", view_box.as_ref().map(view_box_to_string))])),
            SvgBasicMutation::SetTransform { path, transform } => protocol::MutationOutcome::new(attributes_diff_at_path(base, path, &[("transform", transform.as_ref().map(|ops| transform_list_to_string(ops)))])),
        }
    }

    fn inverse(&self, base: &SvgSnapshot) -> Vec<Self> {
        match self {
            SvgBasicMutation::NoMutation => vec![SvgBasicMutation::NoMutation],
            SvgBasicMutation::SetSnapshot { .. } => vec![SvgBasicMutation::SetSnapshot { snapshot: base.clone() }],
            SvgBasicMutation::StampBaseProfile { .. } => vec![SvgBasicMutation::StampBaseProfile { base_profile: prior_attribute(base, &[], "baseProfile"), version: prior_attribute(base, &[], "version") }],
            SvgBasicMutation::InsertBasicElement { parent, index, .. } => vec![SvgBasicMutation::RemoveElement { parent: parent.clone(), index: *index }],
            SvgBasicMutation::RemoveElement { parent, index } => match node_at(&base.doc, parent) {
                Ok(XmlNode::Element { children, .. }) => match children.get(*index) {
                    Some(node) => vec![SvgBasicMutation::InsertBasicElement { parent: parent.clone(), index: *index, node: node.clone() }],
                    None => vec![SvgBasicMutation::NoMutation],
                },
                _ => vec![SvgBasicMutation::NoMutation],
            },
            SvgBasicMutation::SetBasicAttribute { path, name, .. } => vec![SvgBasicMutation::SetBasicAttribute { path: path.clone(), name: name.clone(), value: prior_attribute(base, path, name) }],
            SvgBasicMutation::SetClipPathReference { path, .. } => vec![SvgBasicMutation::SetBasicAttribute { path: path.clone(), name: "clip-path".into(), value: prior_attribute(base, path, "clip-path") }],
            SvgBasicMutation::InsertClipPathShape { clip_path_id, index, .. } => match path_of_id(base, clip_path_id) {
                Some(target) => vec![SvgBasicMutation::RemoveElement { parent: target, index: *index }],
                None => vec![SvgBasicMutation::NoMutation],
            },
            SvgBasicMutation::SetText { path, .. } => {
                let old = match node_at(&base.doc, path) {
                    Ok(XmlNode::Text { text }) => text.clone(),
                    _ => String::new(),
                };
                vec![SvgBasicMutation::SetText { path: path.clone(), text: old }]
            }
            SvgBasicMutation::SetViewBox { path, .. } => vec![SvgBasicMutation::SetViewBox { path: path.clone(), view_box: prior_attribute(base, path, "viewBox").and_then(|v| parse_view_box(&v).ok()) }],
            SvgBasicMutation::SetTransform { path, .. } => vec![SvgBasicMutation::SetTransform { path: path.clone(), transform: prior_attribute(base, path, "transform").and_then(|v| parse_transform_list(&v).ok()) }],
        }
    }
}
//#endregion 🔖️MutationTrait

#[cfg(test)]
mod tests {
    use super::*;

    fn elem(name: &str, attrs: Vec<(&str, &str)>, children: Vec<XmlNode>) -> XmlNode {
        XmlNode::Element { name: name.into(), attrs: attrs.into_iter().map(|(n, v)| XmlAttr { name: n.into(), value: v.into() }).collect(), children }
    }

    fn document() -> SvgSnapshot {
        let mut snapshot = SvgSnapshot::default();
        snapshot.doc.root = Some(elem(
            "svg",
            vec![],
            vec![
                elem("defs", vec![], vec![elem("clipPath", vec![("id", "shape")], vec![elem("path", vec![("d", "M0 0H8V8H0Z")], vec![])]), elem("clipPath", vec![("id", "lettering")], vec![elem("text", vec![], vec![])])]),
                elem("g", vec![], vec![elem("rect", vec![], vec![])]),
            ],
        ));
        snapshot
    }

    /// 📇️ The one test that keeps `KINDS` honest against the enum it claims to spell.
    #[test]
    fn kinds_matches_enum_variants_and_manifest() {
        let every = vec![
            SvgBasicMutation::NoMutation,
            SvgBasicMutation::SetSnapshot { snapshot: SvgSnapshot::default() },
            SvgBasicMutation::StampBaseProfile { base_profile: None, version: None },
            SvgBasicMutation::InsertBasicElement { parent: Vec::new(), index: 0, node: elem("rect", vec![], vec![]) },
            SvgBasicMutation::RemoveElement { parent: Vec::new(), index: 0 },
            SvgBasicMutation::SetBasicAttribute { path: Vec::new(), name: "fill".into(), value: None },
            SvgBasicMutation::SetClipPathReference { path: Vec::new(), clip_path_id: None },
            SvgBasicMutation::InsertClipPathShape { clip_path_id: "shape".into(), index: 0, node: elem("circle", vec![], vec![]) },
            SvgBasicMutation::SetText { path: Vec::new(), text: String::new() },
            SvgBasicMutation::SetViewBox { path: Vec::new(), view_box: None },
            SvgBasicMutation::SetTransform { path: Vec::new(), transform: None },
        ];
        let spelled: Vec<&'static str> = every.iter().map(kind_of).collect();
        assert_eq!(spelled, KINDS.to_vec(), "KINDS must spell every variant, in declaration order");

        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "the oracle manifest's catalog does not declare {kind:?}");
        }
    }

    /// 🔗 Holds this module's gate lists against the subset's own conformance checker, so the two
    /// statements of SVG Basic 1.1's excluded vocabulary cannot drift apart.
    #[test]
    fn blocklists_agree_with_the_subset_conformance_checker() {
        use crate::artifacts::svg::standards::v1_1::subsets::basic::schema::check_svg_basic_conformance;
        let hard = |snapshot: &SvgSnapshot| check_svg_basic_conformance(snapshot).into_iter().any(|d| matches!(d.severity, dsl::Severity::Error | dsl::Severity::Fatal));
        let root = |children: Vec<XmlNode>| {
            let mut snapshot = SvgSnapshot::default();
            snapshot.doc.root = Some(elem("svg", vec![("baseProfile", "basic"), ("version", "1.1")], children));
            snapshot
        };
        for name in BLOCKED_FILTER_PRIMITIVES {
            assert!(is_blocked_filter_primitive(name), "{name} must be gated by this module");
            assert!(hard(&root(vec![elem("filter", vec![("id", "f1")], vec![elem(name, vec![], vec![])])])), "the subset conformance checker does not reject <{name}>");
        }
        for name in ["feGaussianBlur", "feBlend", "feFlood", "feOffset", "linearGradient", "clipPath", "mask"] {
            assert!(!is_blocked_filter_primitive(name), "{name} is retained by SVG Basic 1.1");
            assert!(!hard(&root(vec![elem("filter", vec![("id", "f1")], vec![elem(name, vec![], vec![])])])), "the subset conformance checker rejects the retained <{name}>");
        }
        for name in TEXT_ELEMENTS {
            assert!(carries_text(&elem(name, vec![], vec![])), "{name} must count as text for this module's clip gate");
        }
        assert!(hard(&root(vec![elem("clipPath", vec![("id", "c1")], vec![elem("text", vec![], vec![])]), elem("rect", vec![("clip-path", "url(#c1)")], vec![])])), "the subset conformance checker does not reject clipping to text");
        assert!(!hard(&root(vec![elem("clipPath", vec![("id", "c1")], vec![elem("path", vec![], vec![])]), elem("rect", vec![("clip-path", "url(#c1)")], vec![])])), "the subset conformance checker rejects a shape-only clip path");
    }

    #[test]
    fn insert_basic_element_accepts_a_retained_filter_primitive() {
        let mut snapshot = document();
        let outcome = apply_svg_basic_mutation(&mut snapshot, &SvgBasicMutation::InsertBasicElement { parent: vec![0], index: 0, node: elem("filter", vec![("id", "blur")], vec![elem("feGaussianBlur", vec![("stdDeviation", "2")], vec![])]) });
        assert!(outcome.messages().is_empty(), "feGaussianBlur is retained by SVG Basic 1.1: {:?}", outcome.messages());
    }

    #[test]
    fn insert_basic_element_rejects_an_excluded_primitive() {
        let mut snapshot = document();
        let before = snapshot.clone();
        let outcome = apply_svg_basic_mutation(&mut snapshot, &SvgBasicMutation::InsertBasicElement { parent: vec![0], index: 0, node: elem("filter", vec![], vec![elem("feTurbulence", vec![], vec![])]) });
        assert!(!outcome.messages().is_empty(), "feTurbulence is outside SVG Basic 1.1");
        assert_eq!(snapshot, before, "the document must be untouched");
    }

    #[test]
    fn set_clip_path_reference_rejects_a_clip_path_that_clips_to_text() {
        let mut snapshot = document();
        let outcome = apply_svg_basic_mutation(&mut snapshot, &SvgBasicMutation::SetClipPathReference { path: vec![1], clip_path_id: Some("lettering".into()) });
        assert!(!outcome.messages().is_empty(), "SVG Basic 1.1 does not support clipping to text");
    }

    #[test]
    fn set_clip_path_reference_accepts_a_shape_only_clip_path_and_inverts() {
        let base = document();
        let mut snapshot = base.clone();
        let mutation = SvgBasicMutation::SetClipPathReference { path: vec![1], clip_path_id: Some("shape".into()) };
        let undo = Mutation::inverse(&mutation, &base);
        let outcome = apply_svg_basic_mutation(&mut snapshot, &mutation);
        assert!(outcome.messages().is_empty(), "a shape-only clipPath is legal: {:?}", outcome.messages());
        assert_eq!(element_attr(node_at(&snapshot.doc, &[1]).unwrap(), "clip-path"), Some("url(#shape)"));
        for step in &undo {
            apply_svg_basic_mutation(&mut snapshot, step);
        }
        assert_eq!(snapshot, base, "setting and clearing the clip-path reference must restore the document");
    }

    #[test]
    fn insert_clip_path_shape_rejects_a_text_shape() {
        let mut snapshot = document();
        let before = snapshot.clone();
        let outcome = apply_svg_basic_mutation(&mut snapshot, &SvgBasicMutation::InsertClipPathShape { clip_path_id: "shape".into(), index: 0, node: elem("text", vec![], vec![]) });
        assert!(!outcome.messages().is_empty(), "adding text to a clip path would clip to text");
        assert_eq!(snapshot, before, "the document must be untouched");
    }

    #[test]
    fn insert_clip_path_shape_adds_a_real_shape_and_inverts() {
        let base = document();
        let mut snapshot = base.clone();
        let mutation = SvgBasicMutation::InsertClipPathShape { clip_path_id: "shape".into(), index: 0, node: elem("circle", vec![("cx", "4"), ("cy", "4"), ("r", "4")], vec![]) };
        let undo = Mutation::inverse(&mutation, &base);
        apply_svg_basic_mutation(&mut snapshot, &mutation);
        match node_at(&snapshot.doc, &[0, 0]) {
            Ok(XmlNode::Element { children, .. }) => assert_eq!(children.len(), 2, "the clip path must have gained the shape"),
            other => panic!("expected the clipPath element, got {other:?}"),
        }
        for step in &undo {
            apply_svg_basic_mutation(&mut snapshot, step);
        }
        assert_eq!(snapshot, base, "adding and removing the clip shape must restore the document");
    }
}
