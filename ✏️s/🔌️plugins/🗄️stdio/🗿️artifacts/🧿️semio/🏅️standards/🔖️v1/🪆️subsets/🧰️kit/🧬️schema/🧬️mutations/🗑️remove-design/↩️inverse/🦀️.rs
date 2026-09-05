//! ↩️ Inverse for `RemoveDesign`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{SemioKitMutation, add_design, edit_design};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveDesign, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.designs.iter().find(|d| d.id == payload.id) {
        Some(existing) => vec![
            SemioKitMutation::AddDesign(add_design::AddDesign { id: existing.id.clone(), name: existing.name.clone() }),
            SemioKitMutation::EditDesign(edit_design::EditDesign { id: existing.id.clone(), pieces: existing.pieces.clone(), connections: existing.connections.clone() }),
        ],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
