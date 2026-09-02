//! 🔺️ Sparse diff builder for `ChangeDataProperty` — upserts the addressed node's or edge's
//! property against the current scene off `base`.
use crate::artifacts::jack::diff::{diff_replace_content, JackDiff};
use crate::artifacts::jack::{EntityRef, JackSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeDataProperty, base: &JackSnapshot) -> protocol::MutationOutcome<JackDiff> {
    let mut scene = crate::artifacts::jack::jack_working_scene(base);
    let (kind, id) = match &payload.entity {
        EntityRef::Node(id) => ("node", id),
        EntityRef::Edge(id) => ("edge", id),
    };
    let existing_value = match &payload.entity {
        EntityRef::Node(id) => scene.nodes.iter().find(|node| node.id == *id).map(|node| node.properties.get(&payload.key).cloned()),
        EntityRef::Edge(id) => scene.edges.iter().find(|edge| edge.id == *id).map(|edge| edge.properties.get(&payload.key).cloned()),
    };
    let Some(existing_value) = existing_value else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" does not exist.", if kind == "node" { "Node" } else { "Edge" }, id), [id.clone()]);
    };
    if existing_value.as_ref() == Some(&payload.new_value) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("{kind} \"{id}\" property \"{}\" is already \"{:?}\".", payload.key, payload.new_value));
    }
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
    protocol::MutationOutcome::new(diff_replace_content(scene.nodes, scene.edges))
}
//#endregion 🔖️Diff
