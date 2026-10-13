//! ↩️ Inverse for `DeleteProperties`.

use crate::schema::mutations::{create_properties, SemioObjectMutation};
use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(_payload: &super::DeleteProperties, base: &SemioObjectSnapshot) -> Result<Vec<SemioObjectMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match &base.properties {
        Some(existing) => vec![SemioObjectMutation::CreateProperties(create_properties::CreateProperties { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
