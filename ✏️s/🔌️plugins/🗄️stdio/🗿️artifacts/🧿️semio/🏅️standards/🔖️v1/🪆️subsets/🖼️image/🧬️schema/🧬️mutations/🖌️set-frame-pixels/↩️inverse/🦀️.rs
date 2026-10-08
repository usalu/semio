//! ↩️ Inverse for `SetFramePixels`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetFramePixels, base: &SemioImageSnapshot) -> Result<Vec<SemioImageMutation>, semio_framework_value::ValueError> {
    let super::SetFramePixels { index, .. } = payload;
    Ok(vec![match base.frames.get(*index) {
        Some(frame) => SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: *index, rgba8: frame.rgba8.clone() }),
        None => return Ok(Vec::new()),
    }])
}
//#endregion 🔖️Inverse
