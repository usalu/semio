//! ↩️ Inverse for `AddLayer`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::AddLayer, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::AddLayer { layer, .. } = payload;
    Ok(vec![SemioCadMutation::RemoveLayer(remove_layer::RemoveLayer { name: layer.name.clone() })])
}
//#endregion 🔖️Inverse
