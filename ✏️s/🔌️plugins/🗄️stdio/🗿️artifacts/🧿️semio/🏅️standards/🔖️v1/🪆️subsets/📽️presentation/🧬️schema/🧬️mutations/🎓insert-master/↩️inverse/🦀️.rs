//! ↩️ Inverse for `InsertMaster`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertMaster, base: &SemioPresentationSnapshot) -> Result<Vec<SemioPresentationMutation>, semio_framework_value::ValueError> {
    let super::InsertMaster { master } = payload;
    Ok(vec![SemioPresentationMutation::RemoveMaster(remove_master::RemoveMaster { id: master.id.clone() })])
}
//#endregion 🔖️Inverse
