//! ↩️ Inverse for `SetIcc`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetIcc, base: &SemioImageSnapshot) -> Result<Vec<SemioImageMutation>, semio_framework_value::ValueError> {
    Ok(vec![SemioImageMutation::SetIcc(set_icc::SetIcc { icc: base.icc.clone() })])
}
//#endregion 🔖️Inverse
