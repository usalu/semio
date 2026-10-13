//! ↩️ Inverse for `DeleteObject`.

use crate::schema::mutations::{create_object, SemioKitMutation};
use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteObject, base: &SemioKitSnapshot) -> Result<Vec<SemioKitMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.objects.iter().position(|c| c.child_id == payload.child_id) {
        Some(at) => vec![SemioKitMutation::CreateObject(create_object::CreateObject { child_id: base.objects[at].child_id.clone(), target: base.objects[at].target.clone(), at: Some(at) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
