//! ↩️ `ungroup` — undo is `group` re-wrapping the exact same (now contiguous, spliced-in)
//! children with the dissolved `Group`'s own captured transform; a no-op for the same invalid
//! shapes `diff` refuses.

use super::mutation::UngroupNode;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::node_at;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::delete_node::diff::parent_and_index;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{group, SemioDrawingMutation};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Inverse
pub async fn inverse(payload: &UngroupNode, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
    let Some((parent, group_index)) = parent_and_index(&payload.at) else { return Vec::new() };
    match node_at(base, &payload.at) {
        Some(DrawNode::Group { transform, children }) => {
            let indices: Vec<usize> = (group_index..group_index + children.len()).collect();
            vec![SemioDrawingMutation::Group(group::mutation::GroupNodes { parent, indices, transform: *transform })]
        }
        _ => Vec::new(),
    }
}
//#endregion 🔖️Inverse
