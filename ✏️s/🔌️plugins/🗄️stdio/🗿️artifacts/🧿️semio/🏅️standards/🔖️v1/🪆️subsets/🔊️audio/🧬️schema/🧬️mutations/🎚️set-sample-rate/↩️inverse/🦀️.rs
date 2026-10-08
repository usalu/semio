//! ↩️ Inverse for `SetSampleRate`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetSampleRate, base: &SemioAudioSnapshot) -> Result<Vec<SemioAudioMutation>, semio_framework_value::ValueError> {
    Ok(vec![SemioAudioMutation::SetSampleRate(set_sample_rate::SetSampleRate { sample_rate: base.sample_rate })])
}
//#endregion 🔖️Inverse
