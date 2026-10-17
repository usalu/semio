//! ↩️ Inverse for `RotateObject`.

use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(_payload: &super::RotateObject, base: &SemioObjectSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::rotate_object::RotateObject>,
{
    Ok((|| {
    vec![Op::from(super::RotateObject { rotation: base.transform.rotation })]

    })())
}
//#endregion 🔖️Inverse
