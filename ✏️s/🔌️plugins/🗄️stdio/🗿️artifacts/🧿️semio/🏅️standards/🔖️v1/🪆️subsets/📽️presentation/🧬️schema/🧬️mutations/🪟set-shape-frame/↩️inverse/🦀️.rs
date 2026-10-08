//! ↩️ Inverse for `SetShapeFrame`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetShapeFrame, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    let super::SetShapeFrame { slide_index, shape_index, .. } = payload;
    Ok(match shape_at(base, *slide_index, *shape_index) {
        Some(shape) => vec![SemioPresentationMutation::SetShapeFrame(set_shape_frame::SetShapeFrame { slide_index: *slide_index, shape_index: *shape_index, frame: *frame_of(shape) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
