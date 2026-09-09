//! ↩️ Inverse for `DeleteModel`.

use crate::standards::v1::subsets::kit::schema::mutations::{create_model, SemioKitMutation};
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteModel, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.models.iter().find(|c| c.child_id == payload.child_id) {
        Some(existing) => vec![SemioKitMutation::CreateModel(create_model::CreateModel { child_id: existing.child_id.clone(), target: existing.target.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
