//! ↩️ Inverse for `SetSlideLayout`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetSlideLayout, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    let super::SetSlideLayout { index, .. } = payload;
    Ok(match base.slides.get(*index) {
        Some(slide) => vec![SemioPresentationMutation::SetSlideLayout(set_slide_layout::SetSlideLayout { index: *index, layout_id: slide.layout_id.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
