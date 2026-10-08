//! ↩️ Inverse for `SetColorspace`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetColorspace, base: &SemioImageSnapshot) -> Result<Vec<SemioImageMutation>, semio_framework_value::ValueError> {
    Ok(vec![SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace: base.colorspace })])
}
//#endregion 🔖️Inverse
