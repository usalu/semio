//! 🧬️ `SvgBasicMutation` — the SVG Basic 1.1 mutation vocabulary. Handcrafted for THIS subset, not
//! inherited from `✳️any` and not a longer allow-list of `🔬️tiny`'s.
//!
//! SVG Basic 1.1 (W3C Mobile SVG Profiles, REC-SVGMobile-20030114 §SVG Basic 1.1) KEEPS what Tiny
//! drops — gradients, patterns, masks, opacity, the `clipPath` element and the filter mechanism.
//! What it excludes is narrow and specific: nine expensive raster filter primitives, and clipping to
//! text. Those two exclusions are what this vocabulary is built around.
//! [`SvgBasicMutation::InsertBasicElement`] refuses a subtree carrying an excluded primitive;
//! [`SvgBasicMutation::SetClipPathReference`] and [`SvgBasicMutation::InsertClipPathShape`] address
//! clip paths as first-class subjects and refuse anything that would clip to text. None of the three
//! has a counterpart in `🔬️tiny`, whose profile has neither filters nor `clipPath` at all; and
//! `✳️any`'s ungated `SetAttribute`/`InsertElement` can leave the profile in one step.
//!
//! The excluded-vocabulary lists are restated here, beside the vocabulary they gate, because the
//! subset's own `check_svg_basic_conformance` answers a different question — it judges a whole
//! decoded document, while a mutation gate has to judge a candidate SUBTREE and a candidate
//! reference before either enters one.
//! `blocklists_agree_with_the_subset_conformance_checker` below holds the two against each other so
//! they cannot drift apart silently.
//!
//! @see ../../🔣️oracle.json — the catalog `KINDS` below must match exactly.
//! @see ../../../../../../🧪️tests/🔰️mutate-svg-1-1-basic/🥒️.feature — the case that exercises it.

use crate::schema::diff::{diff_at_path, SvgChildAdded, SvgChildrenDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::schema::mutation_support::{attributes_diff_at_path, prior_attribute};
use crate::schema::snapshot::{node_at, NodePath, TransformOp, ViewBox};




use crate::SvgSnapshot;
use protocol::Mutation;
use crate::schema::snapshot::{SvgNode, SvgAttributeValue};

//#region 🔖️Mutations
#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
#[path = "➕insert-basic-element/🦀️.rs"]
pub mod insert_basic_element;
#[path = "📎insert-clip-path-shape/🦀️.rs"]
pub mod insert_clip_path_shape;
#[path = "➖remove-element/🦀️.rs"]
pub mod remove_element;
#[path = "🏷️set-basic-attribute/🦀️.rs"]
pub mod set_basic_attribute;
#[path = "✂️set-clip-path-reference/🦀️.rs"]
pub mod set_clip_path_reference;
/// 📐️ Typed content mutation for `stdio.svg` 1.1/🔰️basic. Nodes are addressed by `NodePath`; clip
/// paths are addressed by their `id`, because that is how a `clip-path="url(#id)"` reference names
/// them and the profile's whole clip-path rule is about what a reference resolves to.
//#region 🔖️Leaves
#[path = "✍️set-text/🦀️.rs"]
pub mod set_text;
#[path = "🔄set-transform/🦀️.rs"]
pub mod set_transform;
#[path = "🖼️set-view-box/🦀️.rs"]
pub mod set_view_box;
#[path = "🪧stamp-base-profile/🦀️.rs"]
pub mod stamp_base_profile;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SvgSnapshot, diff = SvgDiff, schema = "SvgBasicMutation")]
pub enum SvgBasicMutation {
    /// 🏷️ Sets (or, with `None`, clears) the root's `baseProfile`/`version` declaration.
    StampBaseProfile(stamp_base_profile::StampBaseProfile),
    /// ➕️ Inserts `node` as child `index` of the element at `parent`, REJECTED when the subtree
    /// carries one of the raster filter primitives SVG Basic 1.1 excludes, or is a clip path that
    /// clips to text.
    InsertBasicElement(insert_basic_element::InsertBasicElement),
    /// ➖️ Removes child `index` of the element at `parent`.
    RemoveElement(remove_element::RemoveElement),
    /// 🏷️ Sets (or, with `value: None`, removes) attribute `name` on the element at `path`,
    /// REJECTED when it is a `clip-path` whose `url(#id)` resolves to a clip path containing text.
    SetBasicAttribute(set_basic_attribute::SetBasicAttribute),
    /// ✂️ Points the element at `path` at the clip path named by `clip_path_id` (or clears its
    /// `clip-path` with `None`). REJECTED when the named clip path does not exist, is not a
    /// `clipPath`, or contains a text descendant — SVG Basic 1.1 does not support clipping to text.
    SetClipPathReference(set_clip_path_reference::SetClipPathReference),
    /// ➕️ Adds a clipping shape as child `index` of the clip path named by `clip_path_id`.
    /// REJECTED for a shape that is or contains a text element, for the same profile reason.
    InsertClipPathShape(insert_clip_path_shape::InsertClipPathShape),
    /// ✍️ Replaces the literal text of the `Text` node at `path`.
    SetText(set_text::SetText),
    /// 🖼️ Sets (or clears) the typed `viewBox` of the element at `path`.
    SetViewBox(set_view_box::SetViewBox),
    /// 🔄 Sets (or clears) the typed `transform` list of the element at `path`.
    SetTransform(set_transform::SetTransform),
}

/// 📇️ Kebab-case spelling of every `SvgBasicMutation` variant, in declaration order — the exact
/// `kinds` list `../../🔣️oracle.json`'s `mutationCatalogs` entry declares.
pub const KINDS: &[&str] = &["stamp-base-profile", "insert-basic-element", "remove-element", "set-basic-attribute", "set-clip-path-reference", "insert-clip-path-shape", "set-text", "set-view-box", "set-transform"];

crate::impl_serde_op_codec!(SvgBasicMutation, "svg-basic-mutation");

/// 🏷️ The `KINDS` spelling of one mutation's own variant, exhaustively matched.
pub fn kind_of(mutation: &SvgBasicMutation) -> &'static str {
    match mutation {
        SvgBasicMutation::StampBaseProfile(_) => "stamp-base-profile",
        SvgBasicMutation::InsertBasicElement(_) => "insert-basic-element",
        SvgBasicMutation::RemoveElement(_) => "remove-element",
        SvgBasicMutation::SetBasicAttribute(_) => "set-basic-attribute",
        SvgBasicMutation::SetClipPathReference(_) => "set-clip-path-reference",
        SvgBasicMutation::InsertClipPathShape(_) => "insert-clip-path-shape",
        SvgBasicMutation::SetText(_) => "set-text",
        SvgBasicMutation::SetViewBox(_) => "set-view-box",
        SvgBasicMutation::SetTransform(_) => "set-transform",
    }
}
//#endregion 🔖️Mutations

//#region 🔖️Profile
/// 🚫 The expensive raster filter primitives SVG Basic 1.1 excludes.
const BLOCKED_FILTER_PRIMITIVES: &[&str] = &["feConvolveMatrix", "feDisplacementMap", "feTurbulence", "feMorphology", "feDiffuseLighting", "feSpecularLighting", "feDistantLight", "fePointLight", "feSpotLight"];

/// ✍️ The SVG text element kinds — a clip path containing one clips to text.
const TEXT_ELEMENTS: &[&str] = &["text", "tspan", "tref", "textPath"];

const CODE_REJECTED: &str = "mutation.target-mismatch";

fn local_name(name: &str) -> &str {
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
//#endregion 🔖️Profile

//#region 🔖️AttributeHelper
fn insert_child_diff(parent: &[usize], index: usize, node: &SvgNode) -> SvgDiff {
    diff_at_path(parent, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgChildAdded { index, item: node.clone() }] }) }))
}

fn remove_child_diff(parent: &[usize], index: usize) -> SvgDiff {
    diff_at_path(parent, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: vec![index], modified: Vec::new(), added: Vec::new() }) }))
}
//#endregion 🔖️AttributeHelper


//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: the diff is the single semantics source.
#[cfg(test)]
pub fn apply_svg_basic_mutation(snapshot: &mut SvgSnapshot, mutation: &SvgBasicMutation) -> protocol::MutationOutcome<SvgDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::apply_diff(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion 🔖️Apply


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
