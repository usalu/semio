//! ↩️ Inverse for `RemoveRelation`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveRelation, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    let super::RemoveRelation { id } = payload;
    Ok(match base.relations.iter().find(|r| &r.id == id) {
        Some(original) => vec![SemioModelMutation::InsertRelation(insert_relation::InsertRelation { relation: original.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
