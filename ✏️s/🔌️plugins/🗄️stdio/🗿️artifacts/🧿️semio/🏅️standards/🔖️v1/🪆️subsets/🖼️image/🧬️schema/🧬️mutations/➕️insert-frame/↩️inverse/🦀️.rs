//! ↩️ Inverse for `InsertFrame`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_frame;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertFrame, base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_frame::RemoveFrame>,
{
    let super::InsertFrame { index, .. } = payload;
    Ok(vec![Op::from(remove_frame::RemoveFrame { index: (*index).min(base.frames.len()) })])
}
//#endregion 🔖️Inverse
