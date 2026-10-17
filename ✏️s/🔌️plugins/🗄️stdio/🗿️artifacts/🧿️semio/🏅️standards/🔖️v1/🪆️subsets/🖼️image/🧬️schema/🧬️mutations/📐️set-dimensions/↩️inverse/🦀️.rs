//! ↩️ Inverse for `SetDimensions`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::set_dimensions;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(_payload: &super::SetDimensions, base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_dimensions::SetDimensions>,
{
    Ok(vec![Op::from(set_dimensions::SetDimensions { width: base.width, height: base.height })])
}
//#endregion 🔖️Inverse
