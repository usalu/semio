//! ↩️ Inverse for `MoveFrame`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::MoveFrame, base: &SemioImageSnapshot) -> Result<Vec<SemioImageMutation>, semio_framework_value::ValueError> {
    let super::MoveFrame { from, to } = payload;
    Ok(vec![SemioImageMutation::MoveFrame(move_frame::MoveFrame { from: *to, to: *from })])
}
//#endregion 🔖️Inverse
