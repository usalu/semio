//! ↩️ `rotate` — undo restores the `Group`'s BASE-state rotation; a no-op (`Vec::new()`) for
//! every other node kind or an absent node.

use super::mutation::Rotate;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::node_at;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &Rotate, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match node_at(base, &payload.at) {
        Some(DrawNode::Group { transform, .. }) => vec![SemioDrawingMutation::Rotate(Rotate { at: payload.at.clone(), new_rotation: transform.rotation })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
