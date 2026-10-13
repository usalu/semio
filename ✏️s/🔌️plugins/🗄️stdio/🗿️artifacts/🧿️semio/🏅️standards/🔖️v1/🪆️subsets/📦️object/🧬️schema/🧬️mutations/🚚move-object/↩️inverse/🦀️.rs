//! ↩️ Inverse for `MoveObject`.

use crate::schema::mutations::SemioObjectMutation;
use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(_payload: &super::MoveObject, base: &SemioObjectSnapshot) -> Result<Vec<SemioObjectMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![SemioObjectMutation::MoveObject(super::MoveObject { translation: base.transform.translation })]

    })())
}
//#endregion 🔖️Inverse
