//! ↩️ Inverse for `RemoveMaster`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveMaster, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    let super::RemoveMaster { id } = payload;
    Ok(match master_at(base, id) {
        Some(m) => vec![SemioPresentationMutation::InsertMaster(insert_master::InsertMaster { master: m.clone(), at: base.masters.iter().position(|master| master.id == *id) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
