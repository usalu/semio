//! ↩️ `replace-path` — undo restores the `Path`'s BASE-state `segments`; a no-op (`Vec::new()`)
//! for every other node kind or an absent node.

use super::mutation::ReplacePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::node_at;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &ReplacePath, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match node_at(base, &payload.at) {
        Some(DrawNode::Path { segments, .. }) => vec![SemioDrawingMutation::ReplacePath(ReplacePath { at: payload.at.clone(), new_segments: segments.clone() })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
