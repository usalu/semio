//! ↩️ Inverse for `SetBlockEntityLayer`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetBlockEntityLayer, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::SetBlockEntityLayer { block_name, handle, .. } = payload;
    Ok(match find_block_entity(base, block_name, handle) {
        Some(e) => vec![SemioCadMutation::SetBlockEntityLayer(set_block_entity_layer::SetBlockEntityLayer { block_name: block_name.clone(), handle: handle.clone(), layer: e.layer.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
