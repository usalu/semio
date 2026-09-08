//! 🧬️ `SvgTinyMutation` — the SVG Tiny 1.1 mutation vocabulary. Handcrafted for THIS subset, not
//! inherited from `✳️any`.
//!
//! SVG Tiny 1.1 (W3C Mobile SVG Profiles, REC-SVGMobile-20030114 §SVG Tiny 1.1) is a RESTRICTION of
//! Full 1.1 over the same `SvgSnapshot`. A vocabulary that is honest about that cannot simply be
//! `SvgMutation`: `✳️any`'s `InsertElement`, `SetAttribute` and `SetElementName` can each put a
//! document outside the profile in one step, at which point it is no longer a Tiny document and the
//! subset's own composer refuses to stamp it. Every authoring mutation here is therefore
//! profile-closed — it either preserves Tiny conformance or is rejected with a real diagnostic —
//! and two operations exist that Full 1.1 has no use for at all: [`SvgTinyMutation::StampBaseProfile`],
//! the profile declaration itself, and [`SvgTinyMutation::StripNonTiny`], the Full→Tiny
//! down-conversion.
//!
//! The excluded-vocabulary lists are restated here, beside the vocabulary they gate, because the
//! subset's own `check_svg_tiny_conformance` answers a different question — it judges a whole
//! decoded document, while a mutation gate has to judge a candidate SUBTREE before it enters one.
//! `blocklists_agree_with_the_subset_conformance_checker` below holds the two against each other,
//! element by element and attribute by attribute, so they cannot drift apart silently.
//!
//! @see ../../🔣️oracle.json — the catalog `KINDS` below must match exactly.
//! @see ../../../../../../🧪️tests/🔬️mutate-svg-1-1-tiny/🥒️.feature — the case that exercises it.

use crate::schema::diff::{diff_at_path, diff_set_snapshot, SvgAttrAdded, SvgAttrModified, SvgAttributesDiff, SvgChildAdded, SvgChildrenDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::schema::snapshot::{element_attr, node_at, parse_transform_list, parse_view_box, transform_list_to_string, view_box_to_string, NodePath, TransformOp, ViewBox};
use crate::SvgSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlNode};
use protocol::Mutation;

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.svg` 1.1/🔬️tiny. Nodes are addressed by `NodePath` (a
/// child-index chain from the root `<svg>` element), exactly as the parent subset's own snapshot
/// model does — the snapshot type is shared, only the vocabulary is this subset's.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "🪧stamp-base-profile/🦀️.rs"]
pub mod stamp_base_profile;
#[path = "➕insert-tiny-element/🦀️.rs"]
pub mod insert_tiny_element;
#[path = "➖remove-element/🦀️.rs"]
pub mod remove_element;
#[path = "🏷️set-tiny-attribute/🦀️.rs"]
pub mod set_tiny_attribute;
#[path = "✍️set-text/🦀️.rs"]
pub mod set_text;
#[path = "🖼️set-view-box/🦀️.rs"]
pub mod set_view_box;
#[path = "🔄set-transform/🦀️.rs"]
pub mod set_transform;
#[path = "🧹strip-non-tiny/🦀️.rs"]
pub mod strip_non_tiny;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SvgSnapshot, diff = SvgDiff, schema = "SvgTinyMutation")]
pub enum SvgTinyMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// 🏷️ Sets (or, with `None`, clears) the root's `baseProfile`/`version` declaration. Tiny's own
    /// identity statement; Full 1.1 has no equivalent operation because it has no profile to stamp.
    StampBaseProfile(stamp_base_profile::StampBaseProfile),
    /// ➕️ Inserts `node` as child `index` of the element at `parent`, REJECTED when the subtree
    /// carries an element or presentation attribute SVG Tiny 1.1 excludes.
    InsertTinyElement(insert_tiny_element::InsertTinyElement),
    /// ➖️ Removes child `index` of the element at `parent`. A removal can never leave the profile,
    /// so it needs no gate.
    RemoveElement(remove_element::RemoveElement),
    /// 🏷️ Sets (or, with `value: None`, removes) attribute `name` on the element at `path`,
    /// REJECTED for the presentation attributes SVG Tiny 1.1 forbids everywhere.
    SetTinyAttribute(set_tiny_attribute::SetTinyAttribute),
    /// ✍️ Replaces the literal text of the `Text` node at `path`.
    SetText(set_text::SetText),
    /// 🖼️ Sets (or clears) the typed `viewBox` of the element at `path`.
    SetViewBox(set_view_box::SetViewBox),
    /// 🔄 Sets (or clears) the typed `transform` list of the element at `path`.
    SetTransform(set_transform::SetTransform),
    /// ✂️ The Full→Tiny down-conversion: drops every excluded element subtree and every forbidden
    /// presentation attribute anywhere in the document.
    StripNonTiny(strip_non_tiny::StripNonTiny),
}

/// 📇️ Kebab-case spelling of every `SvgTinyMutation` variant, in declaration order — the exact
/// `kinds` list `../../🔣️oracle.json`'s `mutationCatalogs` entry declares. The framework
/// never parses this enum; `kinds_matches_enum_variants_and_manifest` below is what keeps the two
/// declarations honest against each other.
pub const KINDS: &[&str] = &["set-snapshot", "stamp-base-profile", "insert-tiny-element", "remove-element", "set-tiny-attribute", "set-text", "set-view-box", "set-transform", "strip-non-tiny"];

crate::impl_serde_op_codec!(SvgTinyMutation, "svg-tiny-mutation");

/// 🏷️ The `KINDS` spelling of one mutation's own variant. An exhaustive match (no wildcard arm), so
/// a new variant that forgets its kebab spelling fails to compile rather than failing silently.
pub fn kind_of(mutation: &SvgTinyMutation) -> &'static str {
    match mutation {
        SvgTinyMutation::SetSnapshot(_) => "set-snapshot",
        SvgTinyMutation::StampBaseProfile(_) => "stamp-base-profile",
        SvgTinyMutation::InsertTinyElement(_) => "insert-tiny-element",
        SvgTinyMutation::RemoveElement(_) => "remove-element",
        SvgTinyMutation::SetTinyAttribute(_) => "set-tiny-attribute",
        SvgTinyMutation::SetText(_) => "set-text",
        SvgTinyMutation::SetViewBox(_) => "set-view-box",
        SvgTinyMutation::SetTransform(_) => "set-transform",
        SvgTinyMutation::StripNonTiny(_) => "strip-non-tiny",
    }
}
//#endregion 🔖️Mutations

//#region 🔖️Profile
/// 🚫 Elements SVG Tiny 1.1 excludes outright; `fe*` filter primitives match by prefix, since Tiny
/// forbids the whole filter mechanism.
const BLOCKED_ELEMENTS: &[&str] = &["style", "script", "symbol", "marker", "clipPath", "mask", "pattern", "linearGradient", "radialGradient", "stop", "filter", "cursor", "textPath", "tspan", "tref", "view"];

/// 🚫 Presentation attributes SVG Tiny 1.1 forbids on ANY element.
const BLOCKED_ATTRS: &[&str] = &["style", "opacity", "fill-opacity", "stroke-opacity", "clip-path", "mask", "filter"];

const CODE_REJECTED: &str = "stdio.svg.tiny.mutation-outside-profile";

fn local_name(name: &str) -> &str {
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
pub fn subtree_profile_violation(node: &XmlNode) -> Option<String> {
    match node {
        XmlNode::Element { name, attrs, children } => {
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

/// ✂️ Drops every excluded element subtree and every forbidden presentation attribute below `node`.
fn strip_non_tiny(node: &mut XmlNode) {
    if let XmlNode::Element { attrs, children, .. } = node {
        attrs.retain(|a| !is_blocked_attribute(&a.name));
        children.retain(|c| !matches!(c, XmlNode::Element { name, .. } if is_blocked_element(name)));
        for child in children.iter_mut() {
            strip_non_tiny(child);
        }
    }
}

/// ✂️ The Tiny projection of a whole snapshot — the state `StripNonTiny` targets.
pub fn stripped_to_tiny(base: &SvgSnapshot) -> SvgSnapshot {
    let mut next = base.clone();
    if let Some(root) = next.doc.root.as_mut() {
        strip_non_tiny(root);
    }
    next
}
//#endregion 🔖️Profile

//#region 🔖️AttributeHelper
/// 🏷️ Builds the exact `SvgAttributesDiff` a set of `(name, value)` transitions requires on the
/// element addressed by `path`, resolving each attribute's PRIOR presence against `base`.
/// `added.index` refers to FINAL state, so successive additions count up from the surviving length.
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

/// 🔎 The PRIOR value of attribute `name` on the element addressed by `path` in `base`.
fn prior_attribute(base: &SvgSnapshot, path: &[usize], name: &str) -> Option<String> {
    node_at(&base.doc, path).ok().and_then(|n| element_attr(n, name)).map(|s| s.to_string())
}
//#endregion 🔖️AttributeHelper

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: the diff is the single semantics source, never a separate
/// imperative apply path.
pub fn apply_svg_tiny_mutation(snapshot: &mut SvgSnapshot, mutation: &SvgTinyMutation) -> protocol::MutationOutcome<SvgDiff> {
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
/// vocabulary from outside the crate — an owner-root test adapter, for one — can reach it without
/// naming the `protocol::Mutation` trait, which it has no reason to link.
pub fn inverse_svg_tiny_mutation(mutation: &SvgTinyMutation, base: &SvgSnapshot) -> Vec<SvgTinyMutation> {
    Mutation::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SvgTinyMutation, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
    match this {
        SvgTinyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => protocol::MutationOutcome::new(diff_set_snapshot(base, snapshot)),
        SvgTinyMutation::StampBaseProfile(stamp_base_profile::StampBaseProfile { base_profile, version }) => protocol::MutationOutcome::new(attributes_diff_at_path(base, &[], &[("baseProfile", base_profile.clone()), ("version", version.clone())])),
        SvgTinyMutation::InsertTinyElement(insert_tiny_element::InsertTinyElement { parent, index, node }) => match subtree_profile_violation(node) {
            Some(message) => protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new()),
            None => protocol::MutationOutcome::new(diff_at_path(
                parent,
                SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgChildAdded { index: *index, item: node.clone() }] }) }),
            )),
        },
        SvgTinyMutation::RemoveElement(remove_element::RemoveElement { parent, index }) => protocol::MutationOutcome::new(diff_at_path(
            parent,
            SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }) }),
        )),
        SvgTinyMutation::SetTinyAttribute(set_tiny_attribute::SetTinyAttribute { path, name, value }) => {
            if is_blocked_attribute(name) {
                return protocol::MutationOutcome::error(CODE_REJECTED, format!("attribute '{name}' is forbidden anywhere in SVG Tiny 1.1"), Vec::<String>::new());
            }
            protocol::MutationOutcome::new(attributes_diff_at_path(base, path, &[(name.as_str(), value.clone())]))
        }
        SvgTinyMutation::SetText(set_text::SetText { path, text }) => protocol::MutationOutcome::new(diff_at_path(path, SvgNodeDiff::Text { text: Some(text.clone()) })),
        SvgTinyMutation::SetViewBox(set_view_box::SetViewBox { path, view_box }) => protocol::MutationOutcome::new(attributes_diff_at_path(base, path, &[("viewBox", view_box.as_ref().map(view_box_to_string))])),
        SvgTinyMutation::SetTransform(set_transform::SetTransform { path, transform }) => protocol::MutationOutcome::new(attributes_diff_at_path(base, path, &[("transform", transform.as_ref().map(|ops| transform_list_to_string(ops)))])),
        SvgTinyMutation::StripNonTiny(_) => protocol::MutationOutcome::new(diff_set_snapshot(base, &stripped_to_tiny(base))),
    }
}

/// ↩️ `StripNonTiny` inverts to a whole-document restore, and says so: a strip that removed
/// hundreds of excluded attributes across a real drawing has no smaller undo, and pretending
/// otherwise would be a smaller diff that does not actually restore the document.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SvgTinyMutation, base: &SvgSnapshot) -> Vec<SvgTinyMutation> {
    match this {
        SvgTinyMutation::SetSnapshot(_) | SvgTinyMutation::StripNonTiny(_) => vec![SvgTinyMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        SvgTinyMutation::StampBaseProfile(_) => vec![SvgTinyMutation::StampBaseProfile(stamp_base_profile::StampBaseProfile { base_profile: prior_attribute(base, &[], "baseProfile"), version: prior_attribute(base, &[], "version") })],
        SvgTinyMutation::InsertTinyElement(insert_tiny_element::InsertTinyElement { parent, index, .. }) => vec![SvgTinyMutation::RemoveElement(remove_element::RemoveElement { parent: parent.clone(), index: *index })],
        SvgTinyMutation::RemoveElement(remove_element::RemoveElement { parent, index }) => match node_at(&base.doc, parent) {
            Ok(XmlNode::Element { children, .. }) => match children.get(*index) {
                Some(node) => vec![SvgTinyMutation::InsertTinyElement(insert_tiny_element::InsertTinyElement { parent: parent.clone(), index: *index, node: node.clone() })],
                None => Vec::new(),
            },
            _ => Vec::new(),
        },
        SvgTinyMutation::SetTinyAttribute(set_tiny_attribute::SetTinyAttribute { path, name, .. }) => vec![SvgTinyMutation::SetTinyAttribute(set_tiny_attribute::SetTinyAttribute { path: path.clone(), name: name.clone(), value: prior_attribute(base, path, name) })],
        SvgTinyMutation::SetText(set_text::SetText { path, .. }) => {
            let old = match node_at(&base.doc, path) {
                Ok(XmlNode::Text { text }) => text.clone(),
                _ => String::new(),
            };
            vec![SvgTinyMutation::SetText(set_text::SetText { path: path.clone(), text: old })]
        }
        SvgTinyMutation::SetViewBox(set_view_box::SetViewBox { path, .. }) => vec![SvgTinyMutation::SetViewBox(set_view_box::SetViewBox { path: path.clone(), view_box: prior_attribute(base, path, "viewBox").and_then(|v| parse_view_box(&v).ok()) })],
        SvgTinyMutation::SetTransform(set_transform::SetTransform { path, .. }) => vec![SvgTinyMutation::SetTransform(set_transform::SetTransform { path: path.clone(), transform: prior_attribute(base, path, "transform").and_then(|v| parse_transform_list(&v).ok()) })],
    }
}
//#endregion 🔖️MutationTrait

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
