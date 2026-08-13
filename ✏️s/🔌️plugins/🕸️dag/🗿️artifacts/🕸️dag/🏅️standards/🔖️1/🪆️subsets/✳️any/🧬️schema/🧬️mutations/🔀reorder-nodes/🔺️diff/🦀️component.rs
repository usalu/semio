//! 🔺️ Sparse diff builder for `ReorderNodes`.
use crate::artifacts::dag::diff::DagDiff;
use crate::artifacts::dag::schema::diff::text::diff_replace_content;
use crate::artifacts::dag::{dag_working_scene, DagSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ReorderNodes, base: &DagSnapshot) -> DagDiff {
    let scene = dag_working_scene(base);
    let mut by_id: std::collections::BTreeMap<_, _> = scene.nodes.into_iter().map(|node| (node.id.clone(), node)).collect();
    let mut ordered = Vec::with_capacity(payload.order.len());
    for id in &payload.order {
        if let Some(node) = by_id.remove(id) {
            ordered.push(node);
        }
    }
    ordered.extend(by_id.into_values());
    diff_replace_content(ordered, scene.edges)
}
//#endregion 🔖️Diff
