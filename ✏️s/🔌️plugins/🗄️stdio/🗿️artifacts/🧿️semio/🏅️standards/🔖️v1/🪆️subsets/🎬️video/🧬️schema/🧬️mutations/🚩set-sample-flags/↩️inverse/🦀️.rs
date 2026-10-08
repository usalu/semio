//! ↩️ Inverse for `SetSampleFlags`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetSampleFlags, base: &SemioVideoSnapshot) -> Result<Vec<SemioVideoMutation>, semio_framework_value::ValueError> {
    let super::SetSampleFlags { stream_index, index, .. } = payload;
    Ok(vec![match sample_at(base, *stream_index, *index) {
        Some(sample) => SemioVideoMutation::SetSampleFlags(set_sample_flags::SetSampleFlags { stream_index: *stream_index, index: *index, pts: sample.pts, key: sample.key }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
