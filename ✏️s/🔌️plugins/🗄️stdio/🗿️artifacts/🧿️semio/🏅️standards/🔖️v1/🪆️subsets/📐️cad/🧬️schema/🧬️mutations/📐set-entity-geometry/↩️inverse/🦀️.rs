//! ↩️ Inverse for `SetEntityGeometry`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetEntityGeometry, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::SetEntityGeometry { handle, .. } = payload;
    Ok(match find_entity(base, handle) {
        Some(e) => vec![SemioCadMutation::SetEntityGeometry(set_entity_geometry::SetEntityGeometry { handle: handle.clone(), entity: e.entity.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
