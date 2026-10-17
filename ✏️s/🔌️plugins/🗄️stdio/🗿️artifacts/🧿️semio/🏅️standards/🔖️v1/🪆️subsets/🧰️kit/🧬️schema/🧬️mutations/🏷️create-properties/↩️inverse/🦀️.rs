//! ↩️ Inverse for `CreateProperties`.

use crate::schema::mutations::delete_properties;
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::CreateProperties, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::delete_properties::DeleteProperties>,
{
    Ok((|| {
    if base.properties.is_some() || semio_s_artifact_stdio_semio_base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "value").is_err() {
        return Vec::new();
    }
    vec![Op::from(delete_properties::DeleteProperties {})]

    })())
}
//#endregion 🔖️Inverse
