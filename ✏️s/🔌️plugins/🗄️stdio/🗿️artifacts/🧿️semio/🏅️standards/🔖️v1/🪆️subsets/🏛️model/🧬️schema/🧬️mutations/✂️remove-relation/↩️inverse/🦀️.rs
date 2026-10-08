//! ↩️ Inverse for `RemoveRelation`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveRelation, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    let super::RemoveRelation { id } = payload;
    Ok(match base.relations.iter().position(|r| &r.id == id) {
        Some(at) => vec![SemioModelMutation::InsertRelation(insert_relation::InsertRelation { relation: base.relations[at].clone(), at: Some(at) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
