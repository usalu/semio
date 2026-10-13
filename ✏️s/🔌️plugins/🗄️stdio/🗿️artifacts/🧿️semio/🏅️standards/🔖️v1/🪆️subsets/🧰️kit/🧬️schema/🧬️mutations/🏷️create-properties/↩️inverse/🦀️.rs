//! ↩️ Inverse for `CreateProperties`.

use crate::schema::mutations::{delete_properties, SemioKitMutation};
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::CreateProperties, base: &SemioKitSnapshot) -> Result<Vec<SemioKitMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.properties.is_some() || semio_s_artifact_stdio_semio_base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "value").is_err() {
        return Vec::new();
    }
    vec![SemioKitMutation::DeleteProperties(delete_properties::DeleteProperties {})]

    })())
}
//#endregion 🔖️Inverse
