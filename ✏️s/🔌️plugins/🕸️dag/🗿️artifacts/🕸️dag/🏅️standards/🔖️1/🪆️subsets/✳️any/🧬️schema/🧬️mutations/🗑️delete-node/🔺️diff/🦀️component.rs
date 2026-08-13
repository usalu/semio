//! 🔺️ Sparse diff builder for `DeleteNode` — a real cascade-aware removal (node + any edge that
//! touches it), never a whole-snapshot capture.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::schema::diff::text::diff_replace_content;
use crate::artifacts::dag::schema::split_endpoint;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteNode, base: &DagSnapshot) -> DagDiff {
    let scene = dag_working_scene(base);
    let nodes: Vec<_> = scene.nodes.into_iter().filter(|node| node.id != payload.id).collect();
    let edges: Vec<_> = scene.edges.into_iter().filter(|edge| split_endpoint(&edge.source).0 != payload.id && split_endpoint(&edge.target).0 != payload.id).collect();
    diff_replace_content(nodes, edges)
}
//#endregion 🔖️Diff
