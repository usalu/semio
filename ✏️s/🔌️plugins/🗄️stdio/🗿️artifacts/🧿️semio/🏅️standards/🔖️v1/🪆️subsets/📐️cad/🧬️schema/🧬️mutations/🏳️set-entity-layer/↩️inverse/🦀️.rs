//! ↩️ Inverse for `SetEntityLayer`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetEntityLayer, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::SetEntityLayer { handle, .. } = payload;
    Ok(match find_entity(base, handle) {
        Some(e) => vec![SemioCadMutation::SetEntityLayer(set_entity_layer::SetEntityLayer { handle: handle.clone(), layer: e.layer.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
