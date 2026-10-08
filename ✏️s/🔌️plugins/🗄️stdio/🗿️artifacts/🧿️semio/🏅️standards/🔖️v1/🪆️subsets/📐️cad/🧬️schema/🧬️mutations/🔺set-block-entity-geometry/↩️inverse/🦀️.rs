//! ↩️ Inverse for `SetBlockEntityGeometry`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetBlockEntityGeometry, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::SetBlockEntityGeometry { block_name, handle, .. } = payload;
    Ok(match find_block_entity(base, block_name, handle) {
        Some(e) => vec![SemioCadMutation::SetBlockEntityGeometry(set_block_entity_geometry::SetBlockEntityGeometry { block_name: block_name.clone(), handle: handle.clone(), entity: e.entity.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
