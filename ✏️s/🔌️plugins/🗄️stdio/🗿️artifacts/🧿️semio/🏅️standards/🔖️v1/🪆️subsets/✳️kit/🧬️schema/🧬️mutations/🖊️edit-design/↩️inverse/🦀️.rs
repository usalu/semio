//! ↩️ Inverse for `EditDesign`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitConnection, SemioKitPiece, SemioKitSnapshot};

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::EditDesign, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
    match base.designs.iter().find(|d| d.id == payload.id) {
        Some(existing) => vec![SemioKitMutation::EditDesign(super::EditDesign { id: payload.id.clone(), pieces: existing.pieces.clone(), connections: existing.connections.clone() })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
