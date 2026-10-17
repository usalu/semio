//! ↩️ Inverse for `AddBlockEntity`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_block_entity;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::AddBlockEntity, _base: &SemioCadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_block_entity::RemoveBlockEntity>,
{
    let super::AddBlockEntity { block_name, entity, .. } = payload;
    Ok(vec![Op::from(remove_block_entity::RemoveBlockEntity { block_name: block_name.clone(), handle: entity.handle.clone() })])
}
//#endregion 🔖️Inverse
