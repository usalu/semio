//! ↩️ Inverse for `MoveFrame`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioImageSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::move_frame;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::MoveFrame, _base: &SemioImageSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::move_frame::MoveFrame>,
{
    let super::MoveFrame { from, to } = payload;
    Ok(vec![Op::from(move_frame::MoveFrame { from: *to, to: *from })])
}
//#endregion 🔖️Inverse
