//! 🛠️ Mutation diff helpers: the diff builders for inserting, removing and setting slides, shapes, masters and layouts. See [`super`].

use super::*;

//#region 🔖️MutationDiffHelpers
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_slide_diff(index: usize, sd: SlideDiff) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: None, slides: Some(SlidesDiff { modified: vec![IndexModified { index, diff: sd }], ..Default::default() }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn wrap_shape_diff(slide_index: usize, shape_index: usize, shape_diff: SlideShapeDiff) -> SemioPresentationDiff {
    let shapes_diff = SlideShapesDiff { modified: vec![IndexModified { index: shape_index, diff: shape_diff }], ..Default::default() };
    wrap_slide_diff(slide_index, SlideDiff { id: None, layout_id: None, shapes: Some(shapes_diff), notes: None })
}

/// 🧩 Diff for inserting `🎞️slide` at `index` (FINAL-state index).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_slide(index: usize, slide: Slide) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: None, slides: Some(SlidesDiff { added: vec![IndexAdded { index, item: slide }], ..Default::default() }) }
}

/// 🧱 The positional rows that replace a whole row list: each base row removed at its index, each payload row added at its final index; none when the lists are equal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn replace_rows<T: Clone + PartialEq, D>(old: &[T], new: &[T]) -> Option<IndexedTripleDiff<D, T>> {
    (old != new).then(|| IndexedTripleDiff { removed: (0..old.len()).collect(), modified: Vec::new(), added: new.iter().cloned().enumerate().map(|(index, item)| IndexAdded { index, item }).collect() })
}

/// 🧩 Diff for removing the slide at `index` (BASE-state index).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_slide(index: usize) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: None, slides: Some(SlidesDiff { removed: vec![index], ..Default::default() }) }
}

/// 🧩 Diff for setting (or clearing, `layout_id: None`) slide `index`'s `layout_id`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_slide_layout(base: &SemioPresentationSnapshot, index: usize, layout_id: Option<String>) -> SemioPresentationDiff {
    let Some(slide) = base.slides.get(index) else { return SemioPresentationDiff::default() };
    if slide.layout_id == layout_id {
        return SemioPresentationDiff::default();
    }
    wrap_slide_diff(index, SlideDiff { id: None, layout_id: Some(layout_id), shapes: None, notes: None })
}

/// 🧩 Diff for replacing slide `index`'s `notes`: every base row is removed and the payload rows are added at their positions.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_slide_notes(base: &SemioPresentationSnapshot, index: usize, notes: &[DocBlock]) -> SemioPresentationDiff {
    let Some(slide) = base.slides.get(index) else { return SemioPresentationDiff::default() };
    let Some(notes_diff) = replace_rows(&slide.notes, notes) else { return SemioPresentationDiff::default() };
    wrap_slide_diff(index, SlideDiff { id: None, layout_id: None, shapes: None, notes: Some(notes_diff) })
}

/// 🧩 Diff for inserting `shape` at `shape_index` on slide `slide_index`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_shape(slide_index: usize, shape_index: usize, shape: SlideShape) -> SemioPresentationDiff {
    let shapes_diff = SlideShapesDiff { added: vec![IndexAdded { index: shape_index, item: shape }], ..Default::default() };
    wrap_slide_diff(slide_index, SlideDiff { id: None, layout_id: None, shapes: Some(shapes_diff), notes: None })
}

/// 🧩 Diff for removing the shape at `shape_index` on slide `slide_index`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_shape(slide_index: usize, shape_index: usize) -> SemioPresentationDiff {
    let shapes_diff = SlideShapesDiff { removed: vec![shape_index], ..Default::default() };
    wrap_slide_diff(slide_index, SlideDiff { id: None, layout_id: None, shapes: Some(shapes_diff), notes: None })
}

/// 🧩 Diff for setting shape `shape_index`'s frame on slide `slide_index`, via a real structural
/// comparison against the shape's current frame.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_shape_frame(base: &SemioPresentationSnapshot, slide_index: usize, shape_index: usize, frame: SlideFrame) -> SemioPresentationDiff {
    let Some(shape) = base.slides.get(slide_index).and_then(|s| s.shapes.get(shape_index)) else { return SemioPresentationDiff::default() };
    let Some(frame_diff) = diff_frame(frame_of(shape), &frame) else { return SemioPresentationDiff::default() };
    wrap_shape_diff(slide_index, shape_index, shape_diff_frame_only(shape, frame_diff))
}

/// 🧩 Diff for replacing a `TextBox` shape's `blocks`: every base row is removed and the payload rows are added at their positions.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_textbox_blocks(base: &SemioPresentationSnapshot, slide_index: usize, shape_index: usize, blocks: &[DocBlock]) -> SemioPresentationDiff {
    let Some(SlideShape::TextBox { blocks: old, .. }) = base.slides.get(slide_index).and_then(|s| s.shapes.get(shape_index)) else {
        return SemioPresentationDiff::default();
    };
    let Some(blocks_diff) = replace_rows(old, blocks) else { return SemioPresentationDiff::default() };
    wrap_shape_diff(slide_index, shape_index, SlideShapeDiff::TextBox { frame: None, blocks: Some(blocks_diff) })
}

/// 🧭️ Read-only accessor: every `SlideShape` variant carries a `frame`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn frame_of(shape: &SlideShape) -> &SlideFrame {
    match shape {
        SlideShape::TextBox { frame, .. } | SlideShape::Picture { frame, .. } | SlideShape::Table { frame, .. } | SlideShape::Placeholder { frame, .. } => frame,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn shape_diff_frame_only(shape: &SlideShape, frame_diff: SlideFrameDiff) -> SlideShapeDiff {
    match shape {
        SlideShape::TextBox { .. } => SlideShapeDiff::TextBox { frame: Some(frame_diff), blocks: None },
        SlideShape::Picture { .. } => SlideShapeDiff::Picture { frame: Some(frame_diff), image: None },
        SlideShape::Table { .. } => SlideShapeDiff::Table { frame: Some(frame_diff), rows: None },
        SlideShape::Placeholder { .. } => SlideShapeDiff::Placeholder { frame: Some(frame_diff), kind: None },
    }
}

/// 🧩 Diff for inserting a master.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_master(base: &SemioPresentationSnapshot, master: SlideMaster, at: Option<usize>) -> SemioPresentationDiff {
    let index = at.map_or(base.masters.len(), |at| at.min(base.masters.len()));
    SemioPresentationDiff { masters: Some(SlideMastersDiff { added: vec![NamedAdded { index, item: master }], ..Default::default() }), layouts: None, slides: None }
}

/// 🧩 Diff for removing the master with id `id`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_master(id: &str) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: Some(SlideMastersDiff { removed: vec![id.to_string()], ..Default::default() }), layouts: None, slides: None }
}

/// 🧩 Diff for inserting a layout.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_layout(base: &SemioPresentationSnapshot, layout: SlideLayout, at: Option<usize>) -> SemioPresentationDiff {
    let index = at.map_or(base.layouts.len(), |at| at.min(base.layouts.len()));
    SemioPresentationDiff { masters: None, layouts: Some(SlideLayoutsDiff { added: vec![NamedAdded { index, item: layout }], ..Default::default() }), slides: None }
}

/// 🧩 Diff for removing the layout with id `id`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_layout(id: &str) -> SemioPresentationDiff {
    SemioPresentationDiff { masters: None, layouts: Some(SlideLayoutsDiff { removed: vec![id.to_string()], ..Default::default() }), slides: None }
}

/// 🧩 Diff for setting a layout's `master_id`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_layout_master(id: &str, master_id: &str) -> SemioPresentationDiff {
    let ld = SlideLayoutDiff { master_id: Some(master_id.to_string()), shapes: None };
    SemioPresentationDiff { masters: None, layouts: Some(SlideLayoutsDiff { modified: vec![NamedModified { key: id.to_string(), diff: ld }], ..Default::default() }), slides: None }
}
//#endregion 🔖️MutationDiffHelpers
