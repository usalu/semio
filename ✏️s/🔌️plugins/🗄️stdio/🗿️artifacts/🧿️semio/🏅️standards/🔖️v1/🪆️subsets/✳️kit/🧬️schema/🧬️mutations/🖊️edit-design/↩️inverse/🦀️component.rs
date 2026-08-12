//! ↩️ `edit-design` — self-inverse: another `edit-design` restoring the BASE-state
//! pieces/connections; empty when the id was absent.

use super::mutation::EditDesign;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &EditDesign, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.designs.iter().find(|d| d.id == payload.id) {
        Some(existing) => vec![SemioKitMutation::EditDesign(EditDesign { id: payload.id.clone(), pieces: existing.pieces.clone(), connections: existing.connections.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
