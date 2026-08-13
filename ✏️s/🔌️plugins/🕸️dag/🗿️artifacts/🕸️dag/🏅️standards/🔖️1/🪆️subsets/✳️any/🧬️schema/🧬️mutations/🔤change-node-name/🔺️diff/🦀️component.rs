//! 🔺️ Sparse diff builder for `ChangeNodeName`.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::schema::diff::text::diff_replace_content;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeNodeName, base: &DagSnapshot) -> DagDiff {
    let scene = dag_working_scene(base);
    let mut nodes = scene.nodes;
    if let Some(node) = nodes.iter_mut().find(|node| node.id == payload.id) {
        node.name = payload.new_name.clone();
    }
    diff_replace_content(nodes, scene.edges)
}
//#endregion 🔖️Diff
