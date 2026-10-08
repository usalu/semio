//! ↩️ Inverse for `RemoveLayer`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveLayer, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::RemoveLayer { name } = payload;
    Ok(match find_layer(base, name) {
        Some(l) => vec![SemioCadMutation::AddLayer(add_layer::AddLayer { layer: l.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
