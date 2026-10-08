//! ↩️ Inverse for `SetSlideNotes`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetSlideNotes, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    let super::SetSlideNotes { index, .. } = payload;
    Ok(match base.slides.get(*index) {
        Some(slide) => vec![SemioPresentationMutation::SetSlideNotes(set_slide_notes::SetSlideNotes { index: *index, notes: slide.notes.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
