//! ↩️ Inverse for `CreateModel`.

use crate::schema::mutations::delete_model;
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::CreateModel, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::delete_model::DeleteModel>,
{
    Ok((|| {
    if base.models.iter().any(|model| model.child_id == payload.child_id) || semio_s_artifact_stdio_semio_base::schema::child::validate_semio_child_identity(&payload.child_id, &payload.target, "model").is_err() {
        return Vec::new();
    }
    vec![Op::from(delete_model::DeleteModel { child_id: payload.child_id.clone() })]

    })())
}
//#endregion 🔖️Inverse
