//! ↩️ Inverse for `RemoveBlock`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::add_block;
#[allow(unused_imports)]
use crate::schema::mutations::support::find_block;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveBlock, base: &SemioCadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::add_block::AddBlock>,
{
    let super::RemoveBlock { name } = payload;
    Ok(match find_block(base, name) {
        Some(b) => vec![Op::from(add_block::AddBlock { block: b.clone(), at: base.blocks.iter().position(|block| block.name == *name) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
