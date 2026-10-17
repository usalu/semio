//! ↩️ Inverse for `SetColorspace`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::set_colorspace;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(_payload: &super::SetColorspace, base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_colorspace::SetColorspace>,
{
    Ok(vec![Op::from(set_colorspace::SetColorspace { colorspace: base.colorspace })])
}
//#endregion 🔖️Inverse
