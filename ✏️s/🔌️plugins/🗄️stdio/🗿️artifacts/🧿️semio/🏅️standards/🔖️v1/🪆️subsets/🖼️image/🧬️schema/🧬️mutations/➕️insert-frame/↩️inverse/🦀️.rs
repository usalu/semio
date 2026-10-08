//! ↩️ Inverse for `InsertFrame`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertFrame, base: &SemioImageSnapshot) -> Result<Vec<SemioImageMutation>, semio_framework_value::ValueError> {
    let super::InsertFrame { index, .. } = payload;
    Ok(vec![SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index: (*index).min(base.frames.len()) })])
}
//#endregion 🔖️Inverse
