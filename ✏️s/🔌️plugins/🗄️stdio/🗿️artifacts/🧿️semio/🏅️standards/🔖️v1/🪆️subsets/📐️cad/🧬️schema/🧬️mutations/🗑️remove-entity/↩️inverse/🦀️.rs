//! ↩️ Inverse for `RemoveEntity`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveEntity, base: &SemioCadSnapshot) -> Result<Vec<SemioCadMutation>, semio_framework_value::ValueError> {
    let super::RemoveEntity { handle } = payload;
    Ok(match find_entity(base, handle) {
        Some(e) => vec![SemioCadMutation::AddEntity(add_entity::AddEntity { entity: e.clone(), at: base.entities.iter().position(|entity| entity.handle == *handle) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
