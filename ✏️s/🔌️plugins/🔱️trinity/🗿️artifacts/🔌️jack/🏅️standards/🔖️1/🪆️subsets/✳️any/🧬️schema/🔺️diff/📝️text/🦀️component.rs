//! 🔺️ Jack artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::jack::schema::diff::*;

use crate::artifacts::jack::schema::diff::{
    JackDiff, JackEdgesDelta, JackNodePatchEntry, JackNodesDelta, JackStringList,
};
use crate::artifacts::jack::schema::JackArtifact;
use crate::artifacts::jack::{JackSnapshot, Node};
use protocol::MutationDiff;
use std::collections::BTreeMap;


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


//#region 🔖️Apply
impl JackDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &JackArtifact) -> JackArtifact {
        let mut next = artifact.clone();
        if let Some(value) = &self.schema {
            next.schema = value.clone();
        }
        if let Some(value) = &self.name {
            next.name = value.clone();
        }
        if let Some(value) = &self.manifest_id {
            next.manifest_id = value.clone();
        }
        if let Some(value) = &self.manifest {
            next.manifest = value.clone();
        }
        if let Some(value) = &self.camera {
            next.camera = value.clone();
        }
        if let Some(delta) = &self.nodes {
            next.nodes = apply_nodes_delta(&next.nodes, delta);
        }
        if let Some(delta) = &self.edges {
            next.edges = apply_edges_delta(&next.edges, delta);
        }
        if let Some(value) = &self.root_node_id {
            next.root_node_id = value.clone();
        }
        if let Some(list) = &self.selected_node_ids {
            next.selected_node_ids = list.values.clone();
        }
        if let Some(value) = &self.active_fixture_id {
            next.active_fixture_id = value.clone();
        }
        if let Some(value) = &self.jack_query {
            next.jack_query = value.clone();
        }
        if let Some(modes) = &self.lod_mode_by_window {
            for (key, value) in modes {
                match value {
                    Some(v) => {
                        next.lod_mode_by_window.insert(key.clone(), v.clone());
                    }
                    None => {
                        next.lod_mode_by_window.remove(key);
                    }
                }
            }
        }
        if let Some(value) = &self.viewport_camera {
            next.viewport_camera = value.clone();
        }
        if let Some(value) = &self.jack_result_json {
            next.jack_result_json = value.clone();
        }
        if let Some(value) = &self.editor_engagement_input {
            next.editor_engagement_input = value.clone();
        }
        if let Some(value) = &self.graph_engagement_input {
            next.graph_engagement_input = value.clone();
        }
        if let Some(value) = &self.results_engagement_input {
            next.results_engagement_input = value.clone();
        }
        if let Some(value) = self.reorganize_epoch {
            next.reorganize_epoch = value;
        }
        if let Some(value) = &self.editor_selection {
            next.editor_selection = value.clone();
        }
        if let Some(value) = self.revision {
            next.revision = value;
        }
        if let Some(value) = &self.locale {
            next.locale = value.clone();
        }
        next
    }
}

/// 🧩 Applies an identified-collection delta to nodes.
pub fn apply_nodes_delta(nodes: &[Node], delta: &JackNodesDelta) -> Vec<Node> {
    let mut next = nodes.to_vec();
    for id in &delta.removed {
        next.retain(|node| &node.id != id);
    }
    for node in &delta.added {
        next.push(node.clone());
    }
    for entry in &delta.patched {
        if let Some(node) = next.iter_mut().find(|node| node.id == entry.id) {
            if let Some(name) = &entry.patch.name {
                node.name = name.clone();
            }
            if let Some(x) = entry.patch.x {
                node.x = x;
            }
            if let Some(y) = entry.patch.y {
                node.y = y;
            }
            if let Some(width) = entry.patch.width {
                node.width = width;
            }
            if let Some(height) = entry.patch.height {
                node.height = height;
            }
            if let Some(key) = &entry.patch.key {
                apply_property_patch(&mut node.properties, key, &entry.patch.value_json);
            }
        }
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: BTreeMap<_, _> = next.into_iter().map(|node| (node.id.clone(), node)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(node) = by_id.remove(id) {
                ordered.push(node);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

/// 🩹 Applies a `key`/`value_json` property patch onto a property bag — `Some(json)` upserts the
/// decoded value, `None` (with `key` present) clears the key.
fn apply_property_patch(properties: &mut crate::artifacts::jack::PropertyBag, key: &str, value_json: &Option<Option<String>>) {
    match value_json {
        Some(Some(json)) => {
            if let Ok(value) = serde_json::from_str::<crate::artifacts::jack::PropertyValue>(json) {
                properties.insert(key.to_string(), value);
            }
        }
        Some(None) | None => {
            properties.remove(key);
        }
    }
}

/// 🧩 Applies an identified-collection delta to edges.
pub fn apply_edges_delta(
    edges: &[crate::artifacts::jack::Edge],
    delta: &JackEdgesDelta,
) -> Vec<crate::artifacts::jack::Edge> {
    let mut next = edges.to_vec();
    for id in &delta.removed {
        next.retain(|edge| &edge.id != id);
    }
    for edge in &delta.added {
        next.push(edge.clone());
    }
    for entry in &delta.patched {
        if let Some(edge) = next.iter_mut().find(|edge| edge.id == entry.id) {
            if let Some(key) = &entry.patch.key {
                apply_property_patch(&mut edge.properties, key, &entry.patch.value_json);
            }
        }
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: BTreeMap<_, _> = next.into_iter().map(|edge| (edge.id.clone(), edge)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(edge) = by_id.remove(id) {
                ordered.push(edge);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

impl MutationDiff<JackSnapshot> for JackDiff {
    fn apply(&self, snapshot: &JackSnapshot) -> JackSnapshot {
        let mut next = snapshot.clone();
        if let Some(value) = &self.schema {
            next.schema = value.clone();
        }
        if let Some(value) = &self.name {
            next.name = value.clone();
        }
        if let Some(value) = &self.manifest_id {
            next.manifest_id = value.clone();
        }
        if let Some(value) = &self.manifest {
            next.manifest = value.clone();
        }
        if let Some(value) = &self.camera {
            next.camera = value.clone();
        }
        if let Some(delta) = &self.nodes {
            next.nodes = apply_nodes_delta(&next.nodes, delta);
        }
        if let Some(delta) = &self.edges {
            next.edges = apply_edges_delta(&next.edges, delta);
        }
        if let Some(value) = &self.root_node_id {
            next.root_node_id = value.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(name);
        take!(manifest_id);
        take!(manifest);
        take!(camera);
        take!(root_node_id);
        take!(selected_node_ids);
        take!(active_fixture_id);
        take!(jack_query);
        take!(lod_mode_by_window);
        take!(viewport_camera);
        take!(jack_result_json);
        take!(editor_engagement_input);
        take!(graph_engagement_input);
        take!(results_engagement_input);
        take!(reorganize_epoch);
        take!(editor_selection);
        take!(revision);
        take!(locale);
        match (&mut self.nodes, other.nodes) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (None, Some(src)) => self.nodes = Some(src),
            _ => {}
        }
        match (&mut self.edges, other.edges) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (None, Some(src)) => self.edges = Some(src),
            _ => {}
        }
    }
}

/// 🏗️ Nodes-added delta.
pub fn diff_nodes_added(nodes: Vec<Node>) -> JackDiff {
    JackDiff {
        nodes: Some(JackNodesDelta { added: nodes, ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Nodes-removed delta.
pub fn diff_nodes_removed(ids: Vec<String>) -> JackDiff {
    JackDiff {
        nodes: Some(JackNodesDelta { removed: ids, ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Nodes-patched delta.
pub fn diff_nodes_patched(patched: Vec<JackNodePatchEntry>) -> JackDiff {
    JackDiff {
        nodes: Some(JackNodesDelta { patched, ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Edges-added delta.
pub fn diff_edges_added(edges: Vec<crate::artifacts::jack::Edge>) -> JackDiff {
    JackDiff {
        edges: Some(JackEdgesDelta { added: edges, ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Edges-removed delta.
pub fn diff_edges_removed(ids: Vec<String>) -> JackDiff {
    JackDiff {
        edges: Some(JackEdgesDelta { removed: ids, ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Edges-patched delta.
pub fn diff_edges_patched(patched: Vec<JackEdgePatchEntry>) -> JackDiff {
    JackDiff {
        edges: Some(JackEdgesDelta { patched, ..Default::default() }),
        ..Default::default()
    }
}

/// 🏗️ Node-removed + cascade-severed-edges-removed delta (`delete-node`'s real cascade capture).
pub fn diff_delete_node(id: String, severed_edge_ids: Vec<String>) -> JackDiff {
    JackDiff {
        nodes: Some(JackNodesDelta { removed: vec![id], ..Default::default() }),
        edges: if severed_edge_ids.is_empty() { None } else { Some(JackEdgesDelta { removed: severed_edge_ids, ..Default::default() }) },
        ..Default::default()
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jack_diff_absorb_merges_node_deltas() {
        let mut diff = JackDiff::default();
        let other = JackDiff {
            nodes: Some(JackNodesDelta {
                added: vec![Node {
                    id: "x".into(),
                    kind: "Piece".into(),
                    name: "x".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 1.0,
                    height: 1.0,
                    properties: Default::default(),
                    ports: vec![],
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        diff.absorb(other);
        assert_eq!(diff.nodes.as_ref().map(|d| d.added.len()), Some(1));
    }
}
//#endregion ️Tests
