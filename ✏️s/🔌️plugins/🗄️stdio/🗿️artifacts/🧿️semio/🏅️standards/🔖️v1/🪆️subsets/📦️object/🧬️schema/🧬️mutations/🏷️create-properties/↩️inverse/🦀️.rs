//! ↩️ Inverse for `CreateProperties`.

use crate::schema::mutations::{delete_properties, SemioObjectMutation};
use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateProperties, base: &SemioObjectSnapshot) -> Result<Vec<SemioObjectMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.properties.is_some() || semio_s_artifact_stdio_semio_base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "value").is_err() {
        return Vec::new();
    }
    vec![SemioObjectMutation::DeleteProperties(delete_properties::DeleteProperties {})]

    })())
}
//#endregion 🔖️Inverse
