//! ↩️ Inverse for `SetDimensions`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetDimensions, base: &SemioImageSnapshot) -> Result<Vec<SemioImageMutation>, semio_framework_value::ValueError> {
    Ok(vec![SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width: base.width, height: base.height })])
}
//#endregion 🔖️Inverse
