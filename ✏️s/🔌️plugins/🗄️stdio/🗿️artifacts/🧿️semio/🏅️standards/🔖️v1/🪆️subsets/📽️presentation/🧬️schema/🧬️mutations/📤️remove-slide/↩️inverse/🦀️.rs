//! ↩️ Inverse for `RemoveSlide`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveSlide, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    let super::RemoveSlide { index } = payload;
    Ok(match base.slides.get(*index) {
        Some(slide) => vec![SemioPresentationMutation::InsertSlide(insert_slide::InsertSlide { index: *index, slide: slide.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
