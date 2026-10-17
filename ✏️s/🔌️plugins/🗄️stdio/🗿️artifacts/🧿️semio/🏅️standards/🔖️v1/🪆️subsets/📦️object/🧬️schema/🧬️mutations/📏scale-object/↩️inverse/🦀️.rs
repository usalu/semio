//! ↩️ Inverse for `ScaleObject`.

use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(_payload: &super::ScaleObject, base: &SemioObjectSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::scale_object::ScaleObject>,
{
    Ok((|| {
    vec![Op::from(super::ScaleObject { scale: base.transform.scale })]

    })())
}
//#endregion 🔖️Inverse
