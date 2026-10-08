//! ↩️ Inverse for `SetStreamMeta`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetStreamMeta, base: &SemioVideoSnapshot) -> Result<Vec<SemioVideoMutation>, semio_framework_value::ValueError> {
    let super::SetStreamMeta { index, .. } = payload;
    Ok(vec![match stream_at(base, *index) {
        Some(stream) => SemioVideoMutation::SetStreamMeta(set_stream_meta::SetStreamMeta { index: *index, kind: stream.kind, codec: stream.codec.clone(), width: stream.width, height: stream.height, rate: stream.rate }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
