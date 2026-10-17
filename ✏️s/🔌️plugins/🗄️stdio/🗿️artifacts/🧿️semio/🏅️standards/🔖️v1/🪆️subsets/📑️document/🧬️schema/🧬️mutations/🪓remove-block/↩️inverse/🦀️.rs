//! ↩️ Inverse for `RemoveBlock`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::block_at;
#[allow(unused_imports)]
use crate::schema::mutations::insert_block;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveBlock, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_block::InsertBlock>,
{
    let super::RemoveBlock { path } = payload;
    Ok(match block_at(base, path) {
        Some(block) => vec![Op::from(insert_block::InsertBlock { path: path.clone(), block: block.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
