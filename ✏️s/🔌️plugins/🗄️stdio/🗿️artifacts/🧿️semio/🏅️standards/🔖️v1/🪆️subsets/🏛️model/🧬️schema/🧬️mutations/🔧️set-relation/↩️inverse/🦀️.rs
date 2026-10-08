//! ↩️ Inverse for `SetRelation`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetRelation, base: &SemioModelSnapshot) -> Result<Vec<SemioModelMutation>, semio_framework_value::ValueError> {
    let super::SetRelation { id, kind, from, to } = payload;
    Ok(match base.relations.iter().find(|r| &r.id == id) {
        Some(original) => {
            vec![SemioModelMutation::SetRelation(set_relation::SetRelation { id: id.clone(), kind: kind.as_ref().map(|_| original.kind.clone()), from: from.as_ref().map(|_| original.from.clone()), to: to.as_ref().map(|_| original.to.clone()) })]
        }
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
