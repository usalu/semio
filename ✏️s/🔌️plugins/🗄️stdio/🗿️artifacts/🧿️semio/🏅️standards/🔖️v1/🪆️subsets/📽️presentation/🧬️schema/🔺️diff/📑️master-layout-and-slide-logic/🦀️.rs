//! 📑️ Master, layout and slide logic: applying, inverting and absorbing master, layout and slide diffs. See [`super`].

use super::*;

//#region 🔖️ValueDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_master(master: &mut SlideMaster, diff: &SlideMasterDiff) {
    if let Some(sd) = &diff.shapes {
        apply_indexed(&mut master.shapes, sd, apply_shape);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_layout(layout: &mut SlideLayout, diff: &SlideLayoutDiff) {
    if let Some(v) = &diff.master_id {
        layout.master_id = v.clone();
    }
    if let Some(sd) = &diff.shapes {
        apply_indexed(&mut layout.shapes, sd, apply_shape);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_slide(slide: &mut Slide, diff: &SlideDiff) {
    if let Some(id) = &diff.id {
        slide.id = id.clone();
    }
    if let Some(v) = &diff.layout_id {
        slide.layout_id = v.clone();
    }
    if let Some(sd) = &diff.shapes {
        apply_indexed(&mut slide.shapes, sd, apply_shape);
    }
    if let Some(nd) = &diff.notes {
        apply_indexed(&mut slide.notes, nd, apply_doc_block);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_slide_to_copy(slide: &Slide, diff: &SlideDiff) -> Slide {
    let mut out = slide.clone();
    apply_slide(&mut out, diff);
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_master(base: &SlideMaster, diff: &SlideMasterDiff) -> SlideMasterDiff {
    SlideMasterDiff { shapes: diff.shapes.as_ref().map(|sd| inverse_indexed(&base.shapes, sd, inverse_shape)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_layout(base: &SlideLayout, diff: &SlideLayoutDiff) -> SlideLayoutDiff {
    SlideLayoutDiff { master_id: diff.master_id.as_ref().map(|_| base.master_id.clone()), shapes: diff.shapes.as_ref().map(|sd| inverse_indexed(&base.shapes, sd, inverse_shape)) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_slide(base: &Slide, diff: &SlideDiff) -> SlideDiff {
    SlideDiff {
        id: diff.id.as_ref().map(|_| base.id.clone()),
        layout_id: diff.layout_id.as_ref().map(|_| base.layout_id.clone()),
        shapes: diff.shapes.as_ref().map(|sd| inverse_indexed(&base.shapes, sd, inverse_shape)),
        notes: diff.notes.as_ref().map(|nd| inverse_indexed(&base.notes, nd, inverse_doc_block)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_master_diff(mut a: SlideMasterDiff, b: SlideMasterDiff) -> SlideMasterDiff {
    a.shapes = absorb_opt(a.shapes.take(), b.shapes, |x, y| absorb_indexed(x, y, absorb_shape, apply_shape_to_copy));
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_layout_diff(mut a: SlideLayoutDiff, b: SlideLayoutDiff) -> SlideLayoutDiff {
    if b.master_id.is_some() {
        a.master_id = b.master_id;
    }
    a.shapes = absorb_opt(a.shapes.take(), b.shapes, |x, y| absorb_indexed(x, y, absorb_shape, apply_shape_to_copy));
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_slide_diff(mut a: SlideDiff, b: SlideDiff) -> SlideDiff {
    if b.id.is_some() {
        a.id = b.id;
    }
    if b.layout_id.is_some() {
        a.layout_id = b.layout_id;
    }
    a.shapes = absorb_opt(a.shapes.take(), b.shapes, |x, y| absorb_indexed(x, y, absorb_shape, apply_shape_to_copy));
    a.notes = absorb_opt(a.notes.take(), b.notes, |x, y| absorb_indexed(x, y, absorb_doc_block, apply_doc_block_to_copy));
    a
}
//#endregion 🔖️StructureDiffLogic
