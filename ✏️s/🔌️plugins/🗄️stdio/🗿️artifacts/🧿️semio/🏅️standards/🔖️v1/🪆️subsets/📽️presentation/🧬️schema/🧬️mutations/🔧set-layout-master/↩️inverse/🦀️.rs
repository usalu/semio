//! ↩️ Inverse for `SetLayoutMaster`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetLayoutMaster, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    let super::SetLayoutMaster { id, .. } = payload;
    Ok(match layout_at(base, id) {
        Some(l) => vec![SemioPresentationMutation::SetLayoutMaster(set_layout_master::SetLayoutMaster { id: id.clone(), master_id: l.master_id.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
