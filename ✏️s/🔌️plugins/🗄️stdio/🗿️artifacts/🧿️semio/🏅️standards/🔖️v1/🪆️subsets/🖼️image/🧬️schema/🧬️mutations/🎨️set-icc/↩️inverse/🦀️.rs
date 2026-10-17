//! ↩️ Inverse for `SetIcc`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::set_icc;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(_payload: &super::SetIcc, base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_icc::SetIcc>,
{
    Ok(vec![Op::from(set_icc::SetIcc { icc: base.icc.clone() })])
}
//#endregion 🔖️Inverse
