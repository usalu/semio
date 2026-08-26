//! 🔺️ Sparse diff builder for `RemoveDataProperty` — clears the addressed node's or edge's
//! property against the current scene off `base`.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::{EntityRef, JackSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveDataProperty, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
    let mut scene = crate::artifacts::jack::jack_working_scene(base);
    let (kind, id) = match &payload.entity {
        EntityRef::Node(id) => ("node", id),
        EntityRef::Edge(id) => ("edge", id),
    };
    let has_key = match &payload.entity {
        EntityRef::Node(id) => scene.nodes.iter().find(|node| node.id == *id).map(|node| node.properties.contains_key(&payload.key)),
        EntityRef::Edge(id) => scene.edges.iter().find(|edge| edge.id == *id).map(|edge| edge.properties.contains_key(&payload.key)),
    };
    let Some(has_key) = has_key else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" does not exist.", if kind == "node" { "Node" } else { "Edge" }, id), [id.clone()]);
    };
    if !has_key {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("{kind} \"{id}\" already has no property \"{}\".", payload.key));
    }
    match &payload.entity {
        EntityRef::Node(id) => {
            if let Some(node) = scene.nodes.iter_mut().find(|node| node.id == *id) {
                node.properties.remove(&payload.key);
            }
        }
        EntityRef::Edge(id) => {
            if let Some(edge) = scene.edges.iter_mut().find(|edge| edge.id == *id) {
                edge.properties.remove(&payload.key);
            }
        }
    }
    protocol::MutationOutcome::new(diff_replace_content(scene.nodes, scene.edges))
}
//#endregion 🔖️Diff
