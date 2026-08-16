//! 🔺️ Sparse diff builder for `DeleteEdge` — a real removal.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::JackSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteEdge, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
    let mut scene = crate::artifacts::jack::jack_working_scene(base);
    if !scene.edges.iter().any(|edge| edge.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    scene.edges.retain(|edge| edge.id != payload.id);
    protocol::MutationOutcome::new(diff_replace_content(scene.nodes, scene.edges))
}
//#endregion 🔖️Diff
