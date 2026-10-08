//! ↩️ Inverse for `AddEntity`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::AddEntity, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::AddEntity { entity, .. } = payload;
    Ok(vec![SemioCadMutation::RemoveEntity(remove_entity::RemoveEntity { handle: entity.handle.clone() })])
}
//#endregion 🔖️Inverse
