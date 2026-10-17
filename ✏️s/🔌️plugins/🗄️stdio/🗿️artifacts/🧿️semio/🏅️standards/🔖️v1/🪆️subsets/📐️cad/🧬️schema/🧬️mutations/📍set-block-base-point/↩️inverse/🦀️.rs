//! ↩️ Inverse for `SetBlockBasePoint`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioCadSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::find_block;
#[allow(unused_imports)]
use crate::schema::mutations::set_block_base_point;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetBlockBasePoint, base: &SemioCadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_block_base_point::SetBlockBasePoint>,
{
    let super::SetBlockBasePoint { name, .. } = payload;
    Ok(match find_block(base, name) {
        Some(b) => vec![Op::from(set_block_base_point::SetBlockBasePoint { name: name.clone(), base_point: b.base_point })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
