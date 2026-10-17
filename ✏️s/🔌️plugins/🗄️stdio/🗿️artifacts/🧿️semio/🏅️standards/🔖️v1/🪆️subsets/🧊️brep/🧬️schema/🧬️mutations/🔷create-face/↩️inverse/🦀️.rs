//! ↩️ Inverse for `CreateFace`.

use crate::schema::mutations::delete_face;
use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::CreateFace, _base: &SemioBrepSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::delete_face::DeleteFace>,
{
    Ok((|| {
    vec![Op::from(delete_face::DeleteFace { id: payload.id.clone() })]

    })())
}
//#endregion 🔖️Inverse
