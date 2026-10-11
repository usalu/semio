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
//! down-conversion, whose inverse is [`SvgTinyMutation::RestoreNonTiny`].
//!
//! The excluded-vocabulary lists are restated here, beside the vocabulary they gate, because the
//! subset's own `check_svg_tiny_conformance` answers a different question — it judges a whole
//! decoded document, while a mutation gate has to judge a candidate SUBTREE before it enters one.
//! `blocklists_agree_with_the_subset_conformance_checker` below holds the two against each other,
//! element by element and attribute by attribute, so they cannot drift apart silently.
//!
//! @see ../../🔣️oracle.json — the catalog `KINDS` below must match exactly.
//! @see ../../../../../../🧪️tests/🔬️mutate-svg-1-1-tiny/🥒️.feature — the case that exercises it.

use crate::schema::diff::{diff_at_path, SvgChildAdded, SvgChildrenDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::schema::mutation_support::{attributes_diff_at_path, prior_attribute};
use crate::schema::snapshot::{node_at, NodePath, TransformOp, ViewBox};




use crate::SvgSnapshot;
use protocol::Mutation;
use crate::schema::snapshot::{SvgNode, SvgAttributeValue};

//#region 🔖️Mutations
#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
#[path = "➕insert-tiny-element/🦀️.rs"]
pub mod insert_tiny_element;
#[path = "➖remove-element/🦀️.rs"]
pub mod remove_element;
#[path = "♻️restore-non-tiny/🦀️.rs"]
pub mod restore_non_tiny;
/// 📐️ Typed content mutation for `stdio.svg` 1.1/🔬️tiny. Nodes are addressed by `NodePath` (a
/// child-index chain from the root `<svg>` element), exactly as the parent subset's own snapshot
/// model does — the snapshot type is shared, only the vocabulary is this subset's.
//#region 🔖️Leaves
#[path = "✍️set-text/🦀️.rs"]
pub mod set_text;
#[path = "🏷️set-tiny-attribute/🦀️.rs"]
pub mod set_tiny_attribute;
#[path = "🔄set-transform/🦀️.rs"]
pub mod set_transform;
#[path = "🖼️set-view-box/🦀️.rs"]
pub mod set_view_box;
#[path = "🪧stamp-base-profile/🦀️.rs"]
pub mod stamp_base_profile;
#[path = "🧹strip-non-tiny/🦀️.rs"]
pub mod strip_non_tiny;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = SvgSnapshot, diff = SvgDiff, schema = "SvgTinyMutation")]
pub enum SvgTinyMutation {
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
    /// ♻️ The inverse of the down-conversion: puts the stripped elements and attributes back at their exact positions.
    RestoreNonTiny(restore_non_tiny::RestoreNonTiny),
}

/// 📇️ Kebab-case spelling of every `SvgTinyMutation` variant, in declaration order — the exact
/// `kinds` list `../../🔣️oracle.json`'s `mutationCatalogs` entry declares. The framework
/// never parses this enum; `kinds_matches_enum_variants_and_manifest` below is what keeps the two
/// declarations honest against each other.
pub const KINDS: &[&str] = &["stamp-base-profile", "insert-tiny-element", "remove-element", "set-tiny-attribute", "set-text", "set-view-box", "set-transform", "strip-non-tiny", "restore-non-tiny"];

crate::impl_serde_op_codec!(SvgTinyMutation, "svg-tiny-mutation");

/// 🏷️ The `KINDS` spelling of one mutation's own variant. An exhaustive match (no wildcard arm), so
/// a new variant that forgets its kebab spelling fails to compile rather than failing silently.
pub fn kind_of(mutation: &SvgTinyMutation) -> &'static str {
    match mutation {
        SvgTinyMutation::StampBaseProfile(_) => "stamp-base-profile",
        SvgTinyMutation::InsertTinyElement(_) => "insert-tiny-element",
        SvgTinyMutation::RemoveElement(_) => "remove-element",
        SvgTinyMutation::SetTinyAttribute(_) => "set-tiny-attribute",
        SvgTinyMutation::SetText(_) => "set-text",
        SvgTinyMutation::SetViewBox(_) => "set-view-box",
        SvgTinyMutation::SetTransform(_) => "set-transform",
        SvgTinyMutation::StripNonTiny(_) => "strip-non-tiny",
        SvgTinyMutation::RestoreNonTiny(_) => "restore-non-tiny",
    }
}
//#endregion 🔖️Mutations

//#region 🔖️Profile
/// 🚫 Elements SVG Tiny 1.1 excludes outright; `fe*` filter primitives match by prefix, since Tiny
/// forbids the whole filter mechanism.
const BLOCKED_ELEMENTS: &[&str] = &["style", "script", "symbol", "marker", "clipPath", "mask", "pattern", "linearGradient", "radialGradient", "stop", "filter", "cursor", "textPath", "tspan", "tref", "view"];

/// 🚫 Presentation attributes SVG Tiny 1.1 forbids on ANY element.
const BLOCKED_ATTRS: &[&str] = &["style", "opacity", "fill-opacity", "stroke-opacity", "clip-path", "mask", "filter"];

const CODE_REJECTED: &str = "mutation.target-mismatch";

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

//#endregion 🔖️Profile

//#region 🔖️AttributeHelper
//#endregion 🔖️AttributeHelper


//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: the diff is the single semantics source, never a separate
/// imperative apply path.
#[cfg(test)]
pub fn apply_svg_tiny_mutation(snapshot: &mut SvgSnapshot, mutation: &SvgTinyMutation) -> protocol::MutationOutcome<SvgDiff> {
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
