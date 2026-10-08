//! ↩️ Inverse for `InsertSlide`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertSlide, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    let super::InsertSlide { index, .. } = payload;
    Ok(vec![SemioPresentationMutation::RemoveSlide(remove_slide::RemoveSlide { index: *index })])
}
//#endregion 🔖️Inverse
