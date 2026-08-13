//! 🔺️ Sparse diff builder for `CreateNode` — a real append-only insert (never a whole-snapshot
//! capture).
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::schema::diff::text::diff_replace_content;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateNode, base: &DagSnapshot) -> DagDiff {
    let scene = dag_working_scene(base);
    let mut nodes = scene.nodes;
    nodes.push(payload.node.clone());
    diff_replace_content(nodes, scene.edges)
}
//#endregion 🔖️Diff
