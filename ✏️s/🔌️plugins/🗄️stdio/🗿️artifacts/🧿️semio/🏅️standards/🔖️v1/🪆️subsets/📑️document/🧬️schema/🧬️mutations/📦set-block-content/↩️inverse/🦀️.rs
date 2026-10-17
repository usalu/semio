//! ↩️ Inverse for `SetBlockContent`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::block_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_block_content;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetBlockContent, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_block_content::SetBlockContent>,
{
    let super::SetBlockContent { path, .. } = payload;
    Ok(match block_at(base, path) {
        Some(block) => vec![Op::from(set_block_content::SetBlockContent { path: path.clone(), block: block.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
