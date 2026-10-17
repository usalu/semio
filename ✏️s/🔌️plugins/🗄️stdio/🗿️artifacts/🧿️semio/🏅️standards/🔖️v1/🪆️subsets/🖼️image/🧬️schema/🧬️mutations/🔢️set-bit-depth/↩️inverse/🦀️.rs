//! ↩️ Inverse for `SetBitDepth`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::set_bit_depth;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(_payload: &super::SetBitDepth, base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_bit_depth::SetBitDepth>,
{
    Ok(vec![Op::from(set_bit_depth::SetBitDepth { bit_depth: base.bit_depth })])
}
//#endregion 🔖️Inverse
