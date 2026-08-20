//! ↩️ `move-node` — undo restores the node's BASE-state origin; a no-op (`Vec::new()`) when the
//! node is absent or has no origin field (`Path`).

use super::mutation::MoveNode;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::node_origin;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &MoveNode, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match node_origin(base, &payload.at).await {
        Some(old_origin) => vec![SemioDrawingMutation::MoveNode(MoveNode { at: payload.at.clone(), new_origin: old_origin })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
