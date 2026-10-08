//! ↩️ Inverse for `SetLayer`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetLayer, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::SetLayer { name, color_index, line_type, visible } = payload;
    Ok(match find_layer(base, name) {
        Some(l) => vec![SemioCadMutation::SetLayer(set_layer::SetLayer {
            name: name.clone(),
            color_index: color_index.as_ref().map(|_| l.color_index),
            line_type: line_type.as_ref().map(|_| l.line_type.clone()),
            visible: visible.as_ref().map(|_| l.visible),
        })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
