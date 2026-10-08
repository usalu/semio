//! ↩️ Inverse for `InsertRelation`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertRelation, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    let super::InsertRelation { relation, .. } = payload;
    Ok(vec![SemioModelMutation::RemoveRelation(remove_relation::RemoveRelation { id: relation.id.clone() })])
}
//#endregion 🔖️Inverse
