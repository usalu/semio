//! ↩️ Inverse for `AddEntity`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_entity;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::AddEntity, _base: &SemioCadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_entity::RemoveEntity>,
{
    let super::AddEntity { entity, .. } = payload;
    Ok(vec![Op::from(remove_entity::RemoveEntity { handle: entity.handle.clone() })])
}
//#endregion 🔖️Inverse
