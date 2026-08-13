//! 🔺️ Sparse diff builder for `DeleteEdge` — a real removal.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteEdge, base: &JackSnapshot) -> JackDiff {
    let mut scene = crate::artifacts::jack::jack_working_scene(base);
    scene.edges.retain(|edge| edge.id != payload.id);
    diff_replace_content(scene.nodes, scene.edges)
}
//#endregion 🔖️Diff
