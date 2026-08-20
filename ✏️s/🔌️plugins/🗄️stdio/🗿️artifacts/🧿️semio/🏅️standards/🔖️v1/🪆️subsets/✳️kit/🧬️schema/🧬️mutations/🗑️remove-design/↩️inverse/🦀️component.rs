//! ↩️ `remove-design` — undo is `[add-design, edit-design]`, restoring the FULL escrowed design
//! (name + pieces + connections) from BASE in two steps; empty when absent.

use super::mutation::RemoveDesign;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::{add_design, edit_design, SemioKitMutation};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &RemoveDesign, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.designs.iter().find(|d| d.id == payload.id) {
        Some(existing) => vec![
            SemioKitMutation::AddDesign(add_design::mutation::AddDesign { id: existing.id.clone(), name: existing.name.clone() }),
            SemioKitMutation::EditDesign(edit_design::mutation::EditDesign { id: existing.id.clone(), pieces: existing.pieces.clone(), connections: existing.connections.clone() }),
        ],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
