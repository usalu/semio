//! ↩️ Inverse for `SetSampleData`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetSampleData, base: &SemioVideoSnapshot) -> Result<Vec<SemioVideoMutation>, semio_framework_value::ValueError> {
    let super::SetSampleData { stream_index, index, .. } = payload;
    Ok(vec![match sample_at(base, *stream_index, *index) {
        Some(sample) => SemioVideoMutation::SetSampleData(set_sample_data::SetSampleData { stream_index: *stream_index, index: *index, data: sample.data.clone() }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
