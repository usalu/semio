//! ↩️ Inverse for `MoveObject`.

use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(_payload: &super::MoveObject, base: &SemioObjectSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::move_object::MoveObject>,
{
    Ok((|| {
    vec![Op::from(super::MoveObject { translation: base.transform.translation })]

    })())
}
//#endregion 🔖️Inverse
