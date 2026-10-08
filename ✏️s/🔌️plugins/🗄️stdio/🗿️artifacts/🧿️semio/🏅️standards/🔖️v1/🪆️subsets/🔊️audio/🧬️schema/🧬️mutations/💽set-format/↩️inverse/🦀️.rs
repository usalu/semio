//! ↩️ Inverse for `SetFormat`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetFormat, base: &SemioAudioSnapshot) -> Result<Vec<SemioAudioMutation>, semio_framework_value::ValueError> {
    Ok(vec![SemioAudioMutation::SetFormat(set_format::SetFormat { format: base.format })])
}
//#endregion 🔖️Inverse
