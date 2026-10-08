//! ↩️ Inverse for `RemoveShape`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveShape, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    let super::RemoveShape { slide_index, shape_index } = payload;
    Ok(match shape_at(base, *slide_index, *shape_index) {
        Some(shape) => vec![SemioPresentationMutation::InsertShape(insert_shape::InsertShape { slide_index: *slide_index, shape_index: *shape_index, shape: shape.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
