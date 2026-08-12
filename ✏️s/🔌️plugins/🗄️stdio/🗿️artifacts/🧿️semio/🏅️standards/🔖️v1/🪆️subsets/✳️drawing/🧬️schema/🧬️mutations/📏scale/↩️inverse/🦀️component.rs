//! ↩️ `scale` — undo restores the `Group`'s BASE-state scale; a no-op (`Vec::new()`) for every
//! other node kind or an absent node.

use super::mutation::Scale;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::node_at;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
pub fn inverse(payload: &Scale, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    match node_at(base, &payload.at) {
        Some(DrawNode::Group { transform, .. }) => vec![SemioDrawingMutation::Scale(Scale { at: payload.at.clone(), new_scale: transform.scale })],
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
