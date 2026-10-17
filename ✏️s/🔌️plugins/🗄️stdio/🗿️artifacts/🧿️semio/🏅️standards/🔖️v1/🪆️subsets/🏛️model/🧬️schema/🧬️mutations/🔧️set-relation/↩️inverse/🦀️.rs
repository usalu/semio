//! ↩️ Inverse for `SetRelation`.

use crate::schema::snapshot::SemioModelSnapshot;
use crate::schema::mutations::set_relation;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetRelation, base: &SemioModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_relation::SetRelation>,
{
    let super::SetRelation { id, kind, from, to } = payload;
    Ok(match base.relations.iter().find(|r| &r.id == id) {
        Some(original) => {
            vec![Op::from(set_relation::SetRelation { id: id.clone(), kind: kind.as_ref().map(|_| original.kind.clone()), from: from.as_ref().map(|_| original.from.clone()), to: to.as_ref().map(|_| original.to.clone()) })]
        }
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
