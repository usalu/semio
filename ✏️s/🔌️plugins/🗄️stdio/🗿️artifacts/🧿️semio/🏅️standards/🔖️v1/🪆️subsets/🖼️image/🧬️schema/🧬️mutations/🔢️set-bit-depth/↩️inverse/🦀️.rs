//! ↩️ Inverse for `SetBitDepth`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetBitDepth, base: &SemioImageSnapshot) -> Result<Vec<SemioImageMutation>, semio_framework_value::ValueError> {
    Ok(vec![SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth: base.bit_depth })])
}
//#endregion 🔖️Inverse
