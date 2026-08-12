//! ↩️ `add-design` — undo is `remove-design` for the just-added `id`.

use super::mutation::AddDesign;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{remove_design, SemioKitMutation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &AddDesign, _base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    vec![SemioKitMutation::RemoveDesign(remove_design::mutation::RemoveDesign { id: payload.id.clone() })]
}
//#endregion 🔖️Inverse
