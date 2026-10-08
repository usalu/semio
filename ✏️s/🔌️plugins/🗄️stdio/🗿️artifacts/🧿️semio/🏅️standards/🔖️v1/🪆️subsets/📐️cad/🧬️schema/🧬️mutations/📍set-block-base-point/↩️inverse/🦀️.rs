//! ↩️ Inverse for `SetBlockBasePoint`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetBlockBasePoint, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::SetBlockBasePoint { name, .. } = payload;
    Ok(match find_block(base, name) {
        Some(b) => vec![SemioCadMutation::SetBlockBasePoint(set_block_base_point::SetBlockBasePoint { name: name.clone(), base_point: b.base_point })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
