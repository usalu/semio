//! ↩️ Inverse for `CreateObject`.

use crate::schema::mutations::{delete_object, SemioKitMutation};
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateObject, base: &SemioKitSnapshot) -> Result<Vec<SemioKitMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.objects.iter().any(|object| object.child_id == payload.child_id) || semio_s_artifact_stdio_semio_base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "object").is_err() {
        return Vec::new();
    }
    vec![SemioKitMutation::DeleteObject(delete_object::DeleteObject { child_id: payload.child_id.clone() })]

    })())
}
//#endregion 🔖️Inverse
