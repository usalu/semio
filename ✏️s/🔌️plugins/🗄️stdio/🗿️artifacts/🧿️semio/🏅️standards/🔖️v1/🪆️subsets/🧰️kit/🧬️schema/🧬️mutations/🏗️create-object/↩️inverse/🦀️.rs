//! ↩️ Inverse for `CreateObject`.

use crate::schema::mutations::delete_object;
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::CreateObject, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::delete_object::DeleteObject>,
{
    Ok((|| {
    if base.objects.iter().any(|object| object.child_id == payload.child_id) || semio_s_artifact_stdio_semio_base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "object").is_err() {
        return Vec::new();
    }
    vec![Op::from(delete_object::DeleteObject { child_id: payload.child_id.clone() })]

    })())
}
//#endregion 🔖️Inverse
