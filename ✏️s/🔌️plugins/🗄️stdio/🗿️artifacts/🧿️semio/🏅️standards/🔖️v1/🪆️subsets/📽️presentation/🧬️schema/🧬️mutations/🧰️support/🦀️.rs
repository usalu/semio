//! 🧰️ `SemioPresentationMutation` support: the diff builders and base accessors its mutation leaves call, hosted below the leaves so that no leaf has to import the aggregate module.

use crate::schema::snapshot::SemioPresentationSnapshot;
use crate::schema::snapshot::SlideLayout;
use crate::schema::snapshot::SlideMaster;
use crate::schema::snapshot::SlideShape;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn shape_at(base: &SemioPresentationSnapshot, slide_index: usize, shape_index: usize) -> Option<&SlideShape> {
    base.slides.get(slide_index)?.shapes.get(shape_index)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn master_at<'a>(base: &'a SemioPresentationSnapshot, id: &str) -> Option<&'a SlideMaster> {
    base.masters.iter().find(|m| m.id == id)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn layout_at<'a>(base: &'a SemioPresentationSnapshot, id: &str) -> Option<&'a SlideLayout> {
    base.layouts.iter().find(|l| l.id == id)
}
