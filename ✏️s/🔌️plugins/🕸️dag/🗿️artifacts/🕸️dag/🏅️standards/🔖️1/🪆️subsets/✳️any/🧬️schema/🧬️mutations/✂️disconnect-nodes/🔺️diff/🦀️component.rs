//! 🔺️ Sparse diff builder for `DisconnectNodes`.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::schema::diff::text::diff_replace_content;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DisconnectNodes, base: &DagSnapshot) -> DagDiff {
    let scene = dag_working_scene(base);
    let edges: Vec<_> = scene.edges.into_iter().filter(|edge| edge.id != payload.id).collect();
    diff_replace_content(scene.nodes, edges)
}
//#endregion 🔖️Diff
