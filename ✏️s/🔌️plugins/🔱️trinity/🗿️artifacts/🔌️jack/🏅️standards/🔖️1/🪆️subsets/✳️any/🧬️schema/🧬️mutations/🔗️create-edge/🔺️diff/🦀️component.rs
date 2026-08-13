//! 🔺️ Sparse diff builder for `CreateEdge` — a real append-only insert.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateEdge, base: &JackSnapshot) -> JackDiff {
    let mut scene = crate::artifacts::jack::jack_working_scene(base);
    scene.edges.push(payload.edge.clone());
    diff_replace_content(scene.nodes, scene.edges)
}
//#endregion 🔖️Diff
