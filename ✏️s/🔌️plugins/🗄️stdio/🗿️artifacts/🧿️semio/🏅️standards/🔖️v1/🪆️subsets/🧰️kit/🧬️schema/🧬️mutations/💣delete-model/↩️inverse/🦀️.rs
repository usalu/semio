//! ↩️ Inverse for `DeleteModel`.

use crate::standards::v1::subsets::kit::schema::mutations::{create_model, SemioKitMutation};
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteModel, base: &SemioKitSnapshot) -> Result<Vec<SemioKitMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.models.iter().position(|c| c.child_id == payload.child_id) {
        Some(at) => vec![SemioKitMutation::CreateModel(create_model::CreateModel { child_id: base.models[at].child_id.clone(), target: base.models[at].target.clone(), at: Some(at) })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
