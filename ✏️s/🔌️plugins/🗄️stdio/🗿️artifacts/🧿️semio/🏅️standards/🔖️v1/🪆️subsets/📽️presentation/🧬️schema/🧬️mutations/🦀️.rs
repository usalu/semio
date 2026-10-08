//! 🧬️ SemioPresentationMutation — presentation-structure mutation dispatch. Every variant's
//! `diff()` is handcrafted (never apply-and-capture) and every variant's `inverse()` is
//! handcrafted, key/index-aware (docx precedent) — expressed as `agg_diff`/`agg_inverse` free
//! functions the `dsl::Mutations` derive's synthesized leaves delegate into, per the stdio
//! mutation-leaf migration recipe.

use crate::standards::v1::subsets::document::schema::snapshot::DocBlock;
use crate::standards::v1::subsets::presentation::schema::diff::{diff_insert_layout, diff_insert_master, diff_insert_shape, diff_insert_slide, diff_remove_layout, diff_remove_master, diff_remove_shape, diff_remove_slide, diff_set_layout_master, diff_set_shape_frame, diff_set_slide_layout, diff_set_slide_notes, diff_set_textbox_blocks, frame_of, SemioPresentationDiff};


















use crate::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot, Slide, SlideFrame, SlideLayout, SlideMaster, SlideShape};
/// 🔧️ `OpBinary`/`OpText` both unconditional (not `#[cfg(test)]`-gated): the real
/// `impl protocol::OpBinary for SemioPresentationMutation` below (production code) calls
/// `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs both traits in scope.
use protocol::{Mutation};

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
/// against (catalog `semio-v1-presentation` in `../../🔣️oracle.json`). 
/// `kinds_match_the_enum_and_the_catalog` keeps it honest against the enum, the manifest and the
/// `💾️binary/📡️.protocol.semio` records that carry each kind's wire tag.
pub const KINDS: &[&str] = &["insert-slide", "remove-slide", "set-slide-layout", "set-slide-notes", "insert-shape", "remove-shape", "set-shape-frame", "set-text-box-blocks", "insert-master", "remove-master", "insert-layout", "remove-layout", "set-layout-master"];
//#endregion 🔖️Mutations

/// 🧮️ Pure diff face of [`Mutation::diff`], named only in this subset's own reachable types (`protocol` is a private
/// `extern crate` alias, so an owner-root test adapter cannot bring the `Mutation` trait into scope).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_semio_presentation_mutation(mutation: &SemioPresentationMutation, base: &SemioPresentationSnapshot) -> protocol::MutationOutcome<SemioPresentationDiff> {
    <SemioPresentationMutation as protocol::Mutation<SemioPresentationSnapshot>>::diff(mutation, base)
}


/// ↩️ `SemioPresentationMutation`'s own computed inverse, reachable from OUTSIDE this crate.
/// `protocol` is a private `extern crate semio_framework_os_kernel as protocol` alias in
/// `🦀️.rs`, so an external caller — an owner-root test adapter is exactly that — cannot bring
/// `protocol::Mutation` into scope and therefore cannot call the trait method at all. This
/// wrapper's signature names only types this subset already exports (`kit`'s precedent for the same
/// structural gap).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_presentation_mutation_inverse(mutation: &SemioPresentationMutation, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    Ok({
    Mutation::inverse(mutation, base)?

    })
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




//#endregion 🔖️MutationTrait

//#region OpCodecs











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
        SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master: SlideMaster { id: "m2".into(), shapes: Vec::new() }, at: None }),
        SemioPresentationMutation::RemoveMaster(remove_master::RemoveMaster { id: "master1".into() }),
        SemioPresentationMutation::InsertLayout(insert_layout::InsertLayout { layout: SlideLayout { id: "l2".into(), master_id: "master1".into(), shapes: Vec::new() }, at: None }),
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

#[cfg(test)]
use protocol::{OpBinary,OpText};
