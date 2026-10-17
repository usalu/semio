//! ↩️ Inverse for `RemoveRelation`.

use crate::schema::snapshot::SemioModelSnapshot;
use crate::schema::mutations::insert_relation;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveRelation, base: &SemioModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_relation::InsertRelation>,
{
    let super::RemoveRelation { id } = payload;
    Ok(match base.relations.iter().position(|r| &r.id == id) {
        Some(at) => vec![Op::from(insert_relation::InsertRelation { relation: base.relations[at].clone(), at: Some(at) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
