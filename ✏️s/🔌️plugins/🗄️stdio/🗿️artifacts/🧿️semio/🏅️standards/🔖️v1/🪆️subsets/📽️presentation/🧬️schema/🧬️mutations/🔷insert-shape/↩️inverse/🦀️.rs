//! ↩️ Inverse for `InsertShape`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertShape, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    let super::InsertShape { slide_index, shape_index, .. } = payload;
    Ok({
        vec![SemioPresentationMutation::RemoveShape(remove_shape::RemoveShape { slide_index: *slide_index, shape_index: *shape_index })]
    })
}
//#endregion 🔖️Inverse
