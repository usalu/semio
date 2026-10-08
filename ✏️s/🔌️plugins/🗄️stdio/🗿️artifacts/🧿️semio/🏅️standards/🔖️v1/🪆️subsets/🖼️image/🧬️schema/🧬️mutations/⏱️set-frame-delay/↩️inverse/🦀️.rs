//! ↩️ Inverse for `SetFrameDelay`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetFrameDelay, base: &SemioImageSnapshot) -> Result<Vec<SemioImageMutation>, semio_framework_value::ValueError> {
    let super::SetFrameDelay { index, .. } = payload;
    Ok(vec![match base.frames.get(*index) {
        Some(frame) => SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: *index, delay_ms: frame.delay_ms }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
