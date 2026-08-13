//! 🔺️ Sparse diff builder for `ResizeNode`.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::schema::diff::text::diff_replace_content;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ResizeNode, base: &DagSnapshot) -> DagDiff {
    let scene = dag_working_scene(base);
    let mut nodes = scene.nodes;
    if let Some(node) = nodes.iter_mut().find(|node| node.id == payload.id) {
        node.width = payload.width;
        node.height = payload.height;
    }
    diff_replace_content(nodes, scene.edges)
}
//#endregion 🔖️Diff
