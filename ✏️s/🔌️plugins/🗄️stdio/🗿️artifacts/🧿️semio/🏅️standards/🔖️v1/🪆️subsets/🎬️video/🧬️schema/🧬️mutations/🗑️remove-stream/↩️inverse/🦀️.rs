//! ↩️ Inverse for `RemoveStream`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveStream, base: &SemioVideoSnapshot) -> Result<Vec<SemioVideoMutation>, semio_framework_value::ValueError> {
    let super::RemoveStream { index } = payload;
    Ok(vec![match stream_at(base, *index) {
        Some(stream) => SemioVideoMutation::InsertStream(insert_stream::InsertStream { index: *index, stream: stream.clone() }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
