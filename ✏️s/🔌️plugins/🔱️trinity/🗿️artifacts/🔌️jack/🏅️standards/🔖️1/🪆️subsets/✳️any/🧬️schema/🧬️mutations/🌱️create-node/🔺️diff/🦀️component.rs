//! 🔺️ Sparse diff builder for `CreateNode` — a real append-only insert (never a whole-snapshot
//! capture). Reads the current scene off `base`, appends the new node to a clone, and replaces the
//! composed content child wholesale via `diff_replace_content`.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateNode, base: &JackSnapshot) -> JackDiff {
    let mut scene = crate::artifacts::jack::jack_working_scene(base);
    scene.nodes.push(payload.node.clone());
    diff_replace_content(scene.nodes, scene.edges)
}
//#endregion 🔖️Diff
