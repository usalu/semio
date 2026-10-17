//! ↩️ Inverse for `InsertMaster`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_master;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertMaster, _base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_master::RemoveMaster>,
{
    let super::InsertMaster { master, .. } = payload;
    Ok(vec![Op::from(remove_master::RemoveMaster { id: master.id.clone() })])
}
//#endregion 🔖️Inverse
