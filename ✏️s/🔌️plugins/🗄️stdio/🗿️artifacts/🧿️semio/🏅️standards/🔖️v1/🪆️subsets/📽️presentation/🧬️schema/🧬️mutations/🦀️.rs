//! 🧬️ SemioPresentationMutation — presentation-structure mutation dispatch. Every variant's
//! `diff()` is handcrafted (never apply-and-capture) and every variant's `inverse()` is
//! handcrafted, key/index-aware (docx precedent) — expressed as `agg_diff`/`agg_inverse` free
//! functions the `dsl::Mutations` derive's synthesized leaves delegate into, per the stdio
//! mutation-leaf migration recipe.

use crate::standards::v1::subsets::document::schema::snapshot::DocBlock;
use crate::standards::v1::subsets::presentation::schema::diff::{
    dec_block, dec_frame, dec_layout, dec_list, dec_master, dec_shape, dec_slide, dec_str, decode_option, diff_insert_layout, diff_insert_master, diff_insert_shape, diff_insert_slide, diff_remove_layout, diff_remove_master, diff_remove_shape,
    diff_remove_slide, diff_set_layout_master, diff_set_shape_frame, diff_set_slide_layout, diff_set_slide_notes, diff_set_snapshot, diff_set_textbox_blocks, enc_block, enc_frame, enc_layout, enc_list, enc_master, enc_shape, enc_slide, enc_str,
    encode_option, frame_of, SemioPresentationDiff,
};
use crate::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot, Slide, SlideFrame, SlideLayout, SlideMaster, SlideShape};
/// 🔧️ `OpBinary`/`OpText` both unconditional (not `#[cfg(test)]`-gated): the real
/// `impl protocol::OpBinary for SemioPresentationMutation` below (production code) calls
/// `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs both traits in scope.
use protocol::{Mutation, OpBinary, OpText};

//#region 🔖️Mutations
#[path = "🧩insert-layout/🦀️.rs"]
pub mod insert_layout;
#[path = "🎓insert-master/🦀️.rs"]
pub mod insert_master;
#[path = "🔷insert-shape/🦀️.rs"]
pub mod insert_shape;
#[path = "🎬insert-slide/🦀️.rs"]
pub mod insert_slide;
#[path = "🧹️remove-layout/🦀️.rs"]
pub mod remove_layout;
#[path = "🚫️remove-master/🦀️.rs"]
pub mod remove_master;
#[path = "🔶remove-shape/🦀️.rs"]
pub mod remove_shape;
#[path = "📤️remove-slide/🦀️.rs"]
pub mod remove_slide;
#[path = "🔧set-layout-master/🦀️.rs"]
pub mod set_layout_master;
#[path = "🪟set-shape-frame/🦀️.rs"]
pub mod set_shape_frame;
#[path = "🧭set-slide-layout/🦀️.rs"]
pub mod set_slide_layout;
#[path = "🧾set-slide-notes/🦀️.rs"]
pub mod set_slide_notes;
/// 📐️ Typed content mutation for `stdio.semio.presentation`. Addresses slides by INDEX (`index`,
/// presentation order), shapes on a slide by `(slide_index, shape_index)` — no recursive path type
/// needed (unlike docx's nested-table `DocxBlockPath`) since a shape tree here is exactly two
/// levels deep. Masters/layouts are addressed by their own `id`.
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "✍️set-text-box-blocks/🦀️.rs"]
pub mod set_textbox_blocks;
//#endregion 🔖️Leaves

/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]`
/// requires every variant to wrap exactly one leaf payload and a unit variant wraps none (the
/// stdio mutation-leaf migration recipe's hard constraint #1 — `no` is also not an approved
/// semantic verb). The `#[value(tag = "mutation", rename_all = "camelCase")]` container attribute
/// is KEPT here, unlike the `tiff` reference this migration was derived from (which carries none):
/// serde's internally tagged representation flattens a newtype variant's struct payload into the
/// same JSON object the tag lives in, so every committed fixture under `📸️set-snapshot/🧪️tests/`
/// and the `📽️mutate-semio-presentation` test adapter's `{"mutation":"insertSlide",...}` vectors keep
/// decoding byte-for-byte unchanged after this migration.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioPresentationSnapshot, diff = SemioPresentationDiff, schema = "SemioPresentationMutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum SemioPresentationMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    /// ➕️ Inserts `🎞️slide` at `index` (FINAL-state index).
    InsertSlide(insert_slide::InsertSlide),
    /// ➖️ Removes the slide at `index` (BASE-state index).
    RemoveSlide(remove_slide::RemoveSlide),
    /// 🔗 Sets (or, if `None`, clears) slide `index`'s `layout_id`.
    SetSlideLayout(set_slide_layout::SetSlideLayout),
    /// 📝️ Replaces slide `index`'s speaker notes wholesale.
    SetSlideNotes(set_slide_notes::SetSlideNotes),
    /// ➕️ Inserts `shape` at `shape_index` on slide `slide_index`.
    InsertShape(insert_shape::InsertShape),
    /// ➖️ Removes the shape at `shape_index` on slide `slide_index`.
    RemoveShape(remove_shape::RemoveShape),
    /// 📐️ Sets shape `shape_index`'s on-slide frame (position/size).
    SetShapeFrame(set_shape_frame::SetShapeFrame),
    /// ✍️ Replaces a `TextBox` shape's `blocks` wholesale.
    SetTextBoxBlocks(set_textbox_blocks::SetTextBoxBlocks),
    /// ➕️ Inserts a master.
    InsertMaster(insert_master::InsertMaster),
    /// ➖️ Removes the master with id `id`.
    RemoveMaster(remove_master::RemoveMaster),
    /// ➕️ Inserts a layout.
    InsertLayout(insert_layout::InsertLayout),
    /// ➖️ Removes the layout with id `id`.
    RemoveLayout(remove_layout::RemoveLayout),
    /// 🔗 Repoints the layout with id `id` to master `master_id`.
    SetLayoutMaster(set_layout_master::SetLayoutMaster),
}

/// 🏷️ This subset's DECLARED mutation vocabulary, kebab-case, in enum declaration order — the one
/// list the repository test platform's completeness gate measures `📽️mutate-semio-presentation`
/// against (catalog `semio-v1-presentation` in `../../🔣️oracle.json`). It aliases
/// [`OP_KEYWORDS`], which the binary op frame's `tag` byte already indexes by [`variant_ordinal`],
/// so the vocabulary is declared exactly once and `kinds_match_the_enum_and_the_catalog` keeps that
/// declaration honest against both the enum and the manifest.
pub const KINDS: &[&str] = &OP_KEYWORDS;
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ `let d = mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d` -- the diff is the
/// single semantics source, never a separate imperative apply path (apply-and-capture is banned).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_presentation_mutation(snapshot: &mut SemioPresentationSnapshot, mutation: &SemioPresentationMutation) -> protocol::MutationOutcome<SemioPresentationDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ `SemioPresentationMutation`'s own computed inverse, reachable from OUTSIDE this crate.
/// `protocol` is a private `extern crate semio_framework_os_kernel as protocol` alias in
/// `🦀️.rs`, so an external caller — an owner-root test adapter is exactly that — cannot bring
/// `protocol::Mutation` into scope and therefore cannot call the trait method at all. This
/// wrapper's signature names only types this subset already exports (`kit`'s precedent for the same
/// structural gap).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_presentation_mutation_inverse(mutation: &SemioPresentationMutation, base: &SemioPresentationSnapshot) -> Vec<SemioPresentationMutation> {
    Mutation::inverse(mutation, base)
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shape_at(base: &SemioPresentationSnapshot, slide_index: usize, shape_index: usize) -> Option<&SlideShape> {
    base.slides.get(slide_index)?.shapes.get(shape_index)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn master_at<'a>(base: &'a SemioPresentationSnapshot, id: &str) -> Option<&'a SlideMaster> {
    base.masters.iter().find(|m| m.id == id)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn layout_at<'a>(base: &'a SemioPresentationSnapshot, id: &str) -> Option<&'a SlideLayout> {
    base.layouts.iter().find(|l| l.id == id)
}
//#endregion 🔖️Helpers

//#region 🔖️MutationTrait
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioPresentationMutation, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    protocol::MutationOutcome::new(match this {
        SemioPresentationMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index, slide }) => diff_insert_slide(*index, slide.clone()),
        SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index }) => diff_remove_slide(*index),
        SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index, layout_id }) => diff_set_slide_layout(base, *index, layout_id.clone()),
        SemioPresentationMutation::SetSlideNotes(set_slide_notes::SetSlideNotes { index, notes }) => diff_set_slide_notes(base, *index, notes),
        SemioPresentationMutation::InsertShape(insert_shape::InsertShape { slide_index, shape_index, shape }) => diff_insert_shape(*slide_index, *shape_index, shape.clone()),
        SemioPresentationMutation::RemoveShape(remove_shape::RemoveShape { slide_index, shape_index }) => diff_remove_shape(*slide_index, *shape_index),
        SemioPresentationMutation::SetShapeFrame(set_shape_frame::SetShapeFrame { slide_index, shape_index, frame }) => diff_set_shape_frame(base, *slide_index, *shape_index, *frame),
        SemioPresentationMutation::SetTextBoxBlocks(set_textbox_blocks::SetTextBoxBlocks { slide_index, shape_index, blocks }) => diff_set_textbox_blocks(base, *slide_index, *shape_index, blocks),
        SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master }) => diff_insert_master(master.clone()),
        SemioPresentationMutation::RemoveMaster(remove_master::RemoveMaster { id }) => diff_remove_master(id),
        SemioPresentationMutation::InsertLayout(insert_layout::InsertLayout { layout }) => diff_insert_layout(layout.clone()),
        SemioPresentationMutation::RemoveLayout(remove_layout::RemoveLayout { id }) => diff_remove_layout(id),
        SemioPresentationMutation::SetLayoutMaster(set_layout_master::SetLayoutMaster { id, master_id }) => diff_set_layout_master(id, master_id),
    })
}

/// ↩️ Lifted verbatim from the former `impl Mutation`, except every `None`/no-match fallback that
/// used to construct `NoMutation` now returns `Vec::new()` (an inverse with nothing to restore) —
/// the convention this migration's fleet coordinator ruled on, since `NoMutation` is no longer a
/// constructible variant.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioPresentationMutation, base: &SemioPresentationSnapshot) -> Vec<SemioPresentationMutation> {
    match this {
        SemioPresentationMutation::SetSnapshot(_) => vec![SemioPresentationMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() })],
        SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index, .. }) => vec![SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index: *index })],
        SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index }) => match base.slides.get(*index) {
            Some(slide) => vec![SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index: *index, slide: slide.clone() })],
            None => Vec::new(),
        },
        SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index, .. }) => match base.slides.get(*index) {
            Some(slide) => vec![SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index: *index, layout_id: slide.layout_id.clone() })],
            None => Vec::new(),
        },
        SemioPresentationMutation::SetSlideNotes(set_slide_notes::SetSlideNotes { index, .. }) => match base.slides.get(*index) {
            Some(slide) => vec![SemioPresentationMutation::SetSlideNotes(set_slide_notes::SetSlideNotes { index: *index, notes: slide.notes.clone() })],
            None => Vec::new(),
        },
        SemioPresentationMutation::InsertShape(insert_shape::InsertShape { slide_index, shape_index, .. }) => {
            vec![SemioPresentationMutation::RemoveShape(remove_shape::RemoveShape { slide_index: *slide_index, shape_index: *shape_index })]
        }
        SemioPresentationMutation::RemoveShape(remove_shape::RemoveShape { slide_index, shape_index }) => match shape_at(base, *slide_index, *shape_index) {
            Some(shape) => vec![SemioPresentationMutation::InsertShape(insert_shape::InsertShape { slide_index: *slide_index, shape_index: *shape_index, shape: shape.clone() })],
            None => Vec::new(),
        },
        SemioPresentationMutation::SetShapeFrame(set_shape_frame::SetShapeFrame { slide_index, shape_index, .. }) => match shape_at(base, *slide_index, *shape_index) {
            Some(shape) => vec![SemioPresentationMutation::SetShapeFrame(set_shape_frame::SetShapeFrame { slide_index: *slide_index, shape_index: *shape_index, frame: *frame_of(shape) })],
            None => Vec::new(),
        },
        SemioPresentationMutation::SetTextBoxBlocks(set_textbox_blocks::SetTextBoxBlocks { slide_index, shape_index, .. }) => match shape_at(base, *slide_index, *shape_index) {
            Some(SlideShape::TextBox { blocks, .. }) => {
                vec![SemioPresentationMutation::SetTextBoxBlocks(set_textbox_blocks::SetTextBoxBlocks { slide_index: *slide_index, shape_index: *shape_index, blocks: blocks.clone() })]
            }
            _ => Vec::new(),
        },
        SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master }) => vec![SemioPresentationMutation::RemoveMaster(remove_master::RemoveMaster { id: master.id.clone() })],
        SemioPresentationMutation::RemoveMaster(remove_master::RemoveMaster { id }) => match master_at(base, id) {
            Some(m) => vec![SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master: m.clone() })],
            None => Vec::new(),
        },
        SemioPresentationMutation::InsertLayout(insert_layout::InsertLayout { layout }) => vec![SemioPresentationMutation::RemoveLayout(remove_layout::RemoveLayout { id: layout.id.clone() })],
        SemioPresentationMutation::RemoveLayout(remove_layout::RemoveLayout { id }) => match layout_at(base, id) {
            Some(l) => vec![SemioPresentationMutation::InsertLayout(insert_layout::InsertLayout { layout: l.clone() })],
            None => Vec::new(),
        },
        SemioPresentationMutation::SetLayoutMaster(set_layout_master::SetLayoutMaster { id, .. }) => match layout_at(base, id) {
            Some(l) => vec![SemioPresentationMutation::SetLayoutMaster(set_layout_master::SetLayoutMaster { id: id.clone(), master_id: l.master_id.clone() })],
            None => Vec::new(),
        },
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🎙️ Hand-rolled `OpText`/`OpBinary` (same reasoning as `DocxMutation`'s: the payload types are
/// data-carrying enums the `dsl::DslOps` derive cannot bridge) — reuses the diff file's
/// `pub(crate)` grammar primitives rather than duplicating them. Grammar: `keyword arg=value ...`
/// (space-separated), matching the docx/gif/svg convention. `no-mutation` is no longer a keyword
/// this codec parses (there is nothing left to construct for it); a `🧪️tests/mutate-*` adapter that
/// must still honor the `no-mutation` scenario id maps it to the identity `set-snapshot` mutation
/// itself, ahead of this codec.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_presentation_mutation(m: &SemioPresentationMutation) -> String {
    match m {
        SemioPresentationMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("set-snapshot snapshot={}", crate::standards::v1::subsets::presentation::schema::diff::enc_presentation_snapshot(snapshot)),
        SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index, slide }) => format!("insert-slide index={index} slide={}", enc_slide(slide)),
        SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index }) => format!("remove-slide index={index}"),
        SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index, layout_id }) => format!("set-slide-layout index={index} layout-id={}", encode_option(layout_id, |v| enc_str(v))),
        SemioPresentationMutation::SetSlideNotes(set_slide_notes::SetSlideNotes { index, notes }) => format!("set-slide-notes index={index} notes={}", enc_list(notes, enc_block)),
        SemioPresentationMutation::InsertShape(insert_shape::InsertShape { slide_index, shape_index, shape }) => format!("insert-shape slide-index={slide_index} shape-index={shape_index} shape={}", enc_shape(shape)),
        SemioPresentationMutation::RemoveShape(remove_shape::RemoveShape { slide_index, shape_index }) => format!("remove-shape slide-index={slide_index} shape-index={shape_index}"),
        SemioPresentationMutation::SetShapeFrame(set_shape_frame::SetShapeFrame { slide_index, shape_index, frame }) => format!("set-shape-frame slide-index={slide_index} shape-index={shape_index} frame={}", enc_frame(frame)),
        SemioPresentationMutation::SetTextBoxBlocks(set_textbox_blocks::SetTextBoxBlocks { slide_index, shape_index, blocks }) => {
            format!("set-textbox-blocks slide-index={slide_index} shape-index={shape_index} blocks={}", enc_list(blocks, enc_block))
        }
        SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master }) => format!("insert-master master={}", enc_master(master)),
        SemioPresentationMutation::RemoveMaster(remove_master::RemoveMaster { id }) => format!("remove-master id={}", enc_str(id)),
        SemioPresentationMutation::InsertLayout(insert_layout::InsertLayout { layout }) => format!("insert-layout layout={}", enc_layout(layout)),
        SemioPresentationMutation::RemoveLayout(remove_layout::RemoveLayout { id }) => format!("remove-layout id={}", enc_str(id)),
        SemioPresentationMutation::SetLayoutMaster(set_layout_master::SetLayoutMaster { id, master_id }) => format!("set-layout-master id={} master-id={}", enc_str(id), enc_str(master_id)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_presentation_mutation(line: &str) -> Result<SemioPresentationMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> =
        rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("presentation mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("presentation mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(SemioPresentationMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: crate::standards::v1::subsets::presentation::schema::diff::dec_presentation_snapshot(arg("snapshot")?)? })),
        "insert-slide" => Ok(SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index: usize_arg("index")?, slide: dec_slide(arg("slide")?)? })),
        "remove-slide" => Ok(SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index: usize_arg("index")? })),
        "set-slide-layout" => Ok(SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index: usize_arg("index")?, layout_id: decode_option(arg("layout-id")?, dec_str)? })),
        "set-slide-notes" => Ok(SemioPresentationMutation::SetSlideNotes(set_slide_notes::SetSlideNotes { index: usize_arg("index")?, notes: dec_list(arg("notes")?, dec_block)? })),
        "insert-shape" => Ok(SemioPresentationMutation::InsertShape(insert_shape::InsertShape { slide_index: usize_arg("slide-index")?, shape_index: usize_arg("shape-index")?, shape: dec_shape(arg("shape")?)? })),
        "remove-shape" => Ok(SemioPresentationMutation::RemoveShape(remove_shape::RemoveShape { slide_index: usize_arg("slide-index")?, shape_index: usize_arg("shape-index")? })),
        "set-shape-frame" => Ok(SemioPresentationMutation::SetShapeFrame(set_shape_frame::SetShapeFrame { slide_index: usize_arg("slide-index")?, shape_index: usize_arg("shape-index")?, frame: dec_frame(arg("frame")?)? })),
        "set-textbox-blocks" => Ok(SemioPresentationMutation::SetTextBoxBlocks(set_textbox_blocks::SetTextBoxBlocks { slide_index: usize_arg("slide-index")?, shape_index: usize_arg("shape-index")?, blocks: dec_list(arg("blocks")?, dec_block)? })),
        "insert-master" => Ok(SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master: dec_master(arg("master")?)? })),
        "remove-master" => Ok(SemioPresentationMutation::RemoveMaster(remove_master::RemoveMaster { id: dec_str(arg("id")?)? })),
        "insert-layout" => Ok(SemioPresentationMutation::InsertLayout(insert_layout::InsertLayout { layout: dec_layout(arg("layout")?)? })),
        "remove-layout" => Ok(SemioPresentationMutation::RemoveLayout(remove_layout::RemoveLayout { id: dec_str(arg("id")?)? })),
        "set-layout-master" => Ok(SemioPresentationMutation::SetLayoutMaster(set_layout_master::SetLayoutMaster { id: dec_str(arg("id")?)?, master_id: dec_str(arg("master-id")?)? })),
        other => Err(format!("presentation mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SemioPresentationMutation {
    fn print_op(&self) -> String {
        print_presentation_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_presentation_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// 🏷️ Ordinal table, same declaration order as `SemioPresentationMutation`'s own enum variants
/// and `parse_presentation_mutation`'s keyword match — the real binary `tag` field's source of
/// truth.
const OP_KEYWORDS: [&str; 14] = [
    "set-snapshot",
    "insert-slide",
    "remove-slide",
    "set-slide-layout",
    "set-slide-notes",
    "insert-shape",
    "remove-shape",
    "set-shape-frame",
    "set-textbox-blocks",
    "insert-master",
    "remove-master",
    "insert-layout",
    "remove-layout",
    "set-layout-master",
];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn variant_ordinal(m: &SemioPresentationMutation) -> u8 {
    match m {
        SemioPresentationMutation::SetSnapshot(_) => 0,
        SemioPresentationMutation::InsertSlide(_) => 1,
        SemioPresentationMutation::RemoveSlide(_) => 2,
        SemioPresentationMutation::SetSlideLayout(_) => 3,
        SemioPresentationMutation::SetSlideNotes(_) => 4,
        SemioPresentationMutation::InsertShape(_) => 5,
        SemioPresentationMutation::RemoveShape(_) => 6,
        SemioPresentationMutation::SetShapeFrame(_) => 7,
        SemioPresentationMutation::SetTextBoxBlocks(_) => 8,
        SemioPresentationMutation::InsertMaster(_) => 9,
        SemioPresentationMutation::RemoveMaster(_) => 10,
        SemioPresentationMutation::InsertLayout(_) => 11,
        SemioPresentationMutation::RemoveLayout(_) => 12,
        SemioPresentationMutation::SetLayoutMaster(_) => 13,
    }
}
/// ✂️ Just the `key=value ...` argument tail of `print_presentation_mutation` — the binary frame's
/// `tag` byte already carries the keyword, so the text keyword itself is redundant in the binary
/// payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_presentation_mutation_args(m: &SemioPresentationMutation) -> String {
    match print_presentation_mutation(m).split_once(' ') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION presentation wave: real binary
/// op frame, replacing the old `print_op().into_bytes()` text-as-binary shortcut. `format u8`
/// (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, see [`OP_KEYWORDS`]) are two
/// REAL fixed fields; the variant's own `key=value ...` argument payload follows as one opaque
/// trailing `bytes` chain — reusing the already-real, already-tested
/// `print_presentation_mutation`/`parse_presentation_mutation` text codec rather than re-deriving a
/// second independent encoding (`protocol-array-of-records`/`protocol-prim-ref-recursion`, per the
/// grammar recipe's own gap table — same honest boundary the sibling `../../🔺️diff/💾️binary/
/// 📡️.protocol.semio` uses).
impl OpBinary for SemioPresentationMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self)];
        out.extend_from_slice(print_presentation_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = OP_KEYWORDS.get(tag as usize).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} out of range for {} declared variants", OP_KEYWORDS.len()) })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword} {args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 Representative `SemioPresentationMutation` cases (one per variant) — single source of truth
/// for this facet's own `op_text_binary_roundtrip_law` AND `ops_grammar_conformance_law`/
/// `protocol_walk_law` in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioPresentationMutation> {
    use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
    use crate::standards::v1::subsets::presentation::schema::snapshot::{PlaceholderKind, SlidePictureImage, SlideTableCell, SlideTableRow};

    let frame = SlideFrame { origin: SemioPoint2 { x: 1.5, y: 2.5 }, width: 3.5, height: 4.5 };
    vec![
        SemioPresentationMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: crate::standards::v1::subsets::presentation::schema::diff::snapshot_b() }),
        SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide {
            index: 1,
            slide: Slide { id: "new".into(), layout_id: Some("layout1".into()), shapes: vec![SlideShape::Table { frame, rows: vec![SlideTableRow { cells: vec![SlideTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] }], notes: Vec::new() },
        }),
        SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index: 0 }),
        SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index: 0, layout_id: Some("other".into()) }),
        SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index: 0, layout_id: None }),
        SemioPresentationMutation::SetSlideNotes(set_slide_notes::SetSlideNotes { index: 1, notes: vec![DocBlock::paragraph("hello world")] }),
        SemioPresentationMutation::InsertShape(insert_shape::InsertShape { slide_index: 0, shape_index: 0, shape: SlideShape::Placeholder { frame, kind: PlaceholderKind::Other { value: "custom".into() } } }),
        SemioPresentationMutation::RemoveShape(remove_shape::RemoveShape { slide_index: 0, shape_index: 0 }),
        SemioPresentationMutation::SetShapeFrame(set_shape_frame::SetShapeFrame { slide_index: 0, shape_index: 0, frame }),
        SemioPresentationMutation::SetTextBoxBlocks(set_textbox_blocks::SetTextBoxBlocks { slide_index: 0, shape_index: 0, blocks: vec![DocBlock::paragraph("changed"), DocBlock::Heading { level: 1, style_id: Some("s".into()), runs: Vec::new() }] }),
        SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master: SlideMaster { id: "m2".into(), shapes: Vec::new() } }),
        SemioPresentationMutation::RemoveMaster(remove_master::RemoveMaster { id: "master1".into() }),
        SemioPresentationMutation::InsertLayout(insert_layout::InsertLayout { layout: SlideLayout { id: "l2".into(), master_id: "master1".into(), shapes: Vec::new() } }),
        SemioPresentationMutation::RemoveLayout(remove_layout::RemoveLayout { id: "layout1".into() }),
        SemioPresentationMutation::SetLayoutMaster(set_layout_master::SetLayoutMaster { id: "layout1".into(), master_id: "master1".into() }),
        SemioPresentationMutation::InsertShape(insert_shape::InsertShape { slide_index: 0, shape_index: 1, shape: SlideShape::Picture { frame, image: SlidePictureImage { asset_id: "x".into(), mime: "image/png".into(), bytes: vec![7, 8] } } }),
    ]
}
//#endregion 🔖️Demo

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📸️set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📸️set-snapshot/🧪️tests/🔬️t074/🦀️.rs"]
mod set_snapshot_rewrites_the_second_slides_textbox_and_adds_a_speaker_note;
//#endregion 🧪️FixtureCases
