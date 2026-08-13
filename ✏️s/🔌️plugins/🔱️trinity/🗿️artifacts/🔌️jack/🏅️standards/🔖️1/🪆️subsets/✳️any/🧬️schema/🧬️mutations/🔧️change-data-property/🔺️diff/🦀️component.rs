//! 🔺️ Sparse diff builder for `ChangeDataProperty` — upserts the addressed node's or edge's
//! property against the current scene off `base`.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::{EntityRef, JackSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeDataProperty, base: &JackSnapshot) -> JackDiff {
    let mut scene = crate::artifacts::jack::jack_working_scene(base);
    match &payload.entity {
        EntityRef::Node(id) => {
            if let Some(node) = scene.nodes.iter_mut().find(|node| node.id == *id) {
                node.properties.insert(payload.key.clone(), payload.new_value.clone());
            }
        }
        EntityRef::Edge(id) => {
            if let Some(edge) = scene.edges.iter_mut().find(|edge| edge.id == *id) {
                edge.properties.insert(payload.key.clone(), payload.new_value.clone());
            }
        }
    }
    diff_replace_content(scene.nodes, scene.edges)
}
//#endregion 🔖️Diff
