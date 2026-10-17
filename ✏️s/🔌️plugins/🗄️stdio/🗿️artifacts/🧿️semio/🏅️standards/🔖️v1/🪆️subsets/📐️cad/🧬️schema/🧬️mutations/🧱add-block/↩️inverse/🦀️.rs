//! ↩️ Inverse for `AddBlock`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_block;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::AddBlock, _base: &SemioCadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_block::RemoveBlock>,
{
    let super::AddBlock { block, .. } = payload;
    Ok(vec![Op::from(remove_block::RemoveBlock { name: block.name.clone() })])
}
//#endregion 🔖️Inverse
