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
#[cfg(any(test, feature = "testkit"))] use protocol::Mutation;
use crate::schema::snapshot::{SvgNode, SvgAttributeValue};

//#region 🔖️Mutations
pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::standards::v1_1::subsets::basic::schema::mutations::support;
pub use support::{BLOCKED_FILTER_PRIMITIVES, TEXT_ELEMENTS, carries_text, is_blocked_filter_primitive, path_of_id, resolve_clip_path, subtree_profile_violation};
#[allow(unused_imports)]
use support::{CODE_REJECTED, insert_child_diff, local_name, remove_child_diff};
#[path = "🧭️edit-rules/🦀️.rs"]
pub mod edit_rules;
pub use semio_s_artifact_stdio_svg_tests::standards::v1_1::subsets::basic::schema::mutations::insert_basic_element;
pub use semio_s_artifact_stdio_svg_mut_family::standards::v1_1::subsets::basic::schema::mutations::insert_clip_path_shape;
pub use semio_s_artifact_stdio_svg_tests::standards::v1_1::subsets::basic::schema::mutations::remove_element;
pub use semio_s_artifact_stdio_svg_tests::standards::v1_1::subsets::basic::schema::mutations::set_basic_attribute;
pub use semio_s_artifact_stdio_svg_mut_family::standards::v1_1::subsets::basic::schema::mutations::set_clip_path_reference;
/// 📐️ Typed content mutation for `stdio.svg` 1.1/🔰️basic. Nodes are addressed by `NodePath`; clip
/// paths are addressed by their `id`, because that is how a `clip-path="url(#id)"` reference names
/// them and the profile's whole clip-path rule is about what a reference resolves to.
//#region 🔖️Leaves
pub use semio_s_artifact_stdio_svg_handcrafted_diff_codec_tests::standards::v1_1::subsets::basic::schema::mutations::set_text;
pub use semio_s_artifact_stdio_svg_tests::standards::v1_1::subsets::basic::schema::mutations::set_transform;
pub use semio_s_artifact_stdio_svg_tests::standards::v1_1::subsets::basic::schema::mutations::set_view_box;
pub use semio_s_artifact_stdio_svg_tests::standards::v1_1::subsets::basic::schema::mutations::stamp_base_profile;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = SvgSnapshot, diff = SvgDiff, schema = "SvgBasicMutation", reference = SvgSnapshot, reference_diff = SvgDiff)]
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




//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: the diff is the single semantics source.
#[cfg(any(test, feature = "testkit"))]
pub fn apply_svg_basic_mutation(snapshot: &mut SvgSnapshot, mutation: &SvgBasicMutation) -> protocol::MutationOutcome<SvgDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::apply_diff_cold(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion 🔖️Apply



