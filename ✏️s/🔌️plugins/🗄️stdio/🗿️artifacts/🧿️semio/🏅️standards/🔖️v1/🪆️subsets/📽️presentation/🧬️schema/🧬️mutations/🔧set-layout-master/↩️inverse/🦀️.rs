//! ↩️ Inverse for `SetLayoutMaster`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::layout_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_layout_master;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetLayoutMaster, base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_layout_master::SetLayoutMaster>,
{
    let super::SetLayoutMaster { id, .. } = payload;
    Ok(match layout_at(base, id) {
        Some(l) => vec![Op::from(set_layout_master::SetLayoutMaster { id: id.clone(), master_id: l.master_id.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
