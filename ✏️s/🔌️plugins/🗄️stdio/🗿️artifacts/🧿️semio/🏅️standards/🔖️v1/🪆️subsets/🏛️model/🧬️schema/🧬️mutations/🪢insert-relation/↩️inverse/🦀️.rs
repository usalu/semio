//! ↩️ Inverse for `InsertRelation`.

use crate::schema::snapshot::SemioModelSnapshot;
use crate::schema::mutations::remove_relation;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertRelation, _base: &SemioModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_relation::RemoveRelation>,
{
    let super::InsertRelation { relation, .. } = payload;
    Ok(vec![Op::from(remove_relation::RemoveRelation { id: relation.id.clone() })])
}
//#endregion 🔖️Inverse
