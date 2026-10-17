//! 🖼️ Frame, image and document block logic: diffing, applying, inverting and absorbing frames, picture images and text box document blocks. See [`super`].

use super::*;

//#region 🔖️ValueDiffLogic
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn diff_frame(old: &SlideFrame, new: &SlideFrame) -> Option<SlideFrameDiff> {
    if old == new {
        return None;
    }
    Some(SlideFrameDiff { origin: (old.origin != new.origin).then_some(new.origin), width: (old.width != new.width).then_some(new.width), height: (old.height != new.height).then_some(new.height) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_frame(frame: &mut SlideFrame, diff: &SlideFrameDiff) {
    if let Some(v) = diff.origin {
        frame.origin = v;
    }
    if let Some(v) = diff.width {
        frame.width = v;
    }
    if let Some(v) = diff.height {
        frame.height = v;
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_frame(base: &SlideFrame, diff: &SlideFrameDiff) -> SlideFrameDiff {
    SlideFrameDiff { origin: diff.origin.map(|_| base.origin), width: diff.width.map(|_| base.width), height: diff.height.map(|_| base.height) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_frame(mut a: SlideFrameDiff, b: &SlideFrameDiff) -> SlideFrameDiff {
    if b.origin.is_some() {
        a.origin = b.origin;
    }
    if b.width.is_some() {
        a.width = b.width;
    }
    if b.height.is_some() {
        a.height = b.height;
    }
    a
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_image(old: &SlidePictureImage, new: &SlidePictureImage) -> Option<SlidePictureImageDiff> {
    if old == new {
        return None;
    }
    Some(SlidePictureImageDiff { asset_id: (old.asset_id != new.asset_id).then(|| new.asset_id.clone()), mime: (old.mime != new.mime).then(|| new.mime.clone()), bytes: (old.bytes != new.bytes).then(|| new.bytes.clone()) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_image(image: &mut SlidePictureImage, diff: &SlidePictureImageDiff) {
    if let Some(v) = &diff.asset_id {
        image.asset_id = v.clone();
    }
    if let Some(v) = &diff.mime {
        image.mime = v.clone();
    }
    if let Some(v) = &diff.bytes {
        image.bytes = v.clone();
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_image(base: &SlidePictureImage, diff: &SlidePictureImageDiff) -> SlidePictureImageDiff {
    SlidePictureImageDiff { asset_id: diff.asset_id.as_ref().map(|_| base.asset_id.clone()), mime: diff.mime.as_ref().map(|_| base.mime.clone()), bytes: diff.bytes.as_ref().map(|_| base.bytes.clone()) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_image(mut a: SlidePictureImageDiff, b: SlidePictureImageDiff) -> SlidePictureImageDiff {
    if b.asset_id.is_some() {
        a.asset_id = b.asset_id;
    }
    if b.mime.is_some() {
        a.mime = b.mime;
    }
    if b.bytes.is_some() {
        a.bytes = b.bytes;
    }
    a
}

/// 🧱️ Whole-value `DocBlock` "diff" (`D = T`, see module doc comment) — never sub-diffed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_doc_block(old: &DocBlock, new: &DocBlock) -> Option<DocBlock> {
    (old != new).then(|| new.clone())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_doc_block(block: &mut DocBlock, diff: &DocBlock) {
    *block = diff.clone();
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn apply_doc_block_to_copy(_block: &DocBlock, diff: &DocBlock) -> DocBlock {
    diff.clone()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn inverse_doc_block(base: &DocBlock, _diff: &DocBlock) -> DocBlock {
    base.clone()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn absorb_doc_block(_a: DocBlock, b: DocBlock) -> DocBlock {
    b
}
//#endregion 🔖️StructureDiffLogic
