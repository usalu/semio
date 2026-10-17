//! ↩️ Inverse for `RemoveMaster`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioPresentationSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::insert_master;
#[allow(unused_imports)]
use crate::schema::mutations::support::master_at;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveMaster, base: &SemioPresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_master::InsertMaster>,
{
    let super::RemoveMaster { id } = payload;
    Ok(match master_at(base, id) {
        Some(m) => vec![Op::from(insert_master::InsertMaster { master: m.clone(), at: base.masters.iter().position(|master| master.id == *id) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
