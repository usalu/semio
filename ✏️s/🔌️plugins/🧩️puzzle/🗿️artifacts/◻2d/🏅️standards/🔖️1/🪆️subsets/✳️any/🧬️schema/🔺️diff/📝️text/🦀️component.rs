//! 🔺️ Puzzle 2d artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::puzzle2d::schema::diff::{
    Puzzle2dDiff, Puzzle2dEdgesDelta, Puzzle2dNodePatchEntry, Puzzle2dNodesDelta, Puzzle2dStringList,
};
use crate::artifacts::puzzle2d::schema::Puzzle2dArtifact;
use crate::artifacts::puzzle2d::{Puzzle2dEdge, Puzzle2dNode, Puzzle2dSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


//#region 🔖️Apply
impl Puzzle2dDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &Puzzle2dArtifact) -> Puzzle2dArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(camera) = &self.camera {
            next.camera = camera.clone();
        }
        if let Some(delta) = &self.nodes {
            next.nodes = apply_nodes_delta(&next.nodes, delta);
        }
        if let Some(delta) = &self.edges {
            next.edges = apply_edges_delta(&next.edges, delta);
        }
        if let Some(meta) = &self.meta {
            next.meta = meta.clone();
        }
        if let Some(list) = &self.selected_ids {
            next.selected_ids = list.values.clone();
        }
        if let Some(value) = &self.active_utility_id {
            next.active_utility_id = value.clone();
        }
        if let Some(value) = self.camera_x { next.camera_x = value; }
        if let Some(value) = self.camera_y { next.camera_y = value; }
        if let Some(value) = self.camera_zoom { next.camera_zoom = value; }
        if let Some(value) = &self.selection_method { next.selection_method = value.clone(); }
        if let Some(value) = self.grid_snap_enabled { next.grid_snap_enabled = value; }
        if let Some(value) = self.grid_factor { next.grid_factor = value; }
        if let Some(value) = self.suggestion_offset { next.suggestion_offset = value; }
        if let Some(value) = self.fill_count { next.fill_count = value; }
        if let Some(value) = self.brush_candidate_index { next.brush_candidate_index = value; }
        if let Some(value) = &self.brush_candidate_source_handle_id { next.brush_candidate_source_handle_id = value.clone(); }
        if let Some(value) = &self.locale { next.locale = value.clone(); }
        if let Some(value) = &self.terminology { next.terminology = value.clone(); }
        if let Some(value) = &self.lod_mode_by_pane_json { next.lod_mode_by_pane_json = value.clone(); }
        if let Some(value) = &self.engagement_input_by_pane_json { next.engagement_input_by_pane_json = value.clone(); }
        if let Some(value) = &self.brush_candidates_json { next.brush_candidates_json = value.clone(); }
        if let Some(value) = &self.node_kind_weights_json { next.node_kind_weights_json = value.clone(); }
        if let Some(value) = &self.handle_kind_weights_json { next.handle_kind_weights_json = value.clone(); }
        if let Some(value) = &self.active_utility_by_window_id_json { next.active_utility_by_window_id_json = value.clone(); }
        if let Some(value) = &self.hovered_node_id { next.hovered_node_id = value.clone(); }
        if let Some(value) = self.preview_seq { next.preview_seq = value; }
        next
    }
}

fn apply_identified_delta<T: Clone>(
    items: &[T],
    removed: &[String],
    added: &[T],
    patched: &[(String, Option<T>)],
    reordered: &Option<Vec<String>>,
    id_of: impl Fn(&T) -> &str,
) -> Vec<T> {
    let mut next = items.to_vec();
    for id in removed {
        next.retain(|item| id_of(item) != id);
    }
    for item in added {
        if let Some(pos) = next.iter().position(|entry| id_of(entry) == id_of(item)) {
            next[pos] = item.clone();
        } else {
            next.push(item.clone());
        }
    }
    for (id, replacement) in patched {
        if let (Some(pos), Some(value)) = (next.iter().position(|entry| id_of(entry) == id), replacement) {
            next[pos] = value.clone();
        }
    }
    if let Some(order) = reordered {
        let mut by_id: std::collections::BTreeMap<_, _> =
            next.into_iter().map(|item| (id_of(&item).to_string(), item)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(item) = by_id.remove(id) {
                ordered.push(item);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

/// 🧩 Applies an identified-collection delta to nodes.
pub fn apply_nodes_delta(nodes: &[Puzzle2dNode], delta: &Puzzle2dNodesDelta) -> Vec<Puzzle2dNode> {
    let patched: Vec<_> = delta.patched.iter().map(|entry| (entry.id.clone(), entry.patch.replacement.clone())).collect();
    apply_identified_delta(nodes, &delta.removed, &delta.added, &patched, &delta.reordered, |n| &n.id)
}

/// 🧩 Applies an identified-collection delta to edges.
pub fn apply_edges_delta(edges: &[Puzzle2dEdge], delta: &Puzzle2dEdgesDelta) -> Vec<Puzzle2dEdge> {
    let patched: Vec<_> = delta.patched.iter().map(|entry| (entry.id.clone(), entry.patch.replacement.clone())).collect();
    apply_identified_delta(edges, &delta.removed, &delta.added, &patched, &delta.reordered, |e| &e.id)
}

impl MutationDiff<Puzzle2dSnapshot> for Puzzle2dDiff {
    fn apply(&self, snapshot: &Puzzle2dSnapshot) -> Puzzle2dSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema { next.schema = schema.clone(); }
        if let Some(camera) = &self.camera { next.camera = camera.clone(); }
        if let Some(delta) = &self.nodes { next.nodes = apply_nodes_delta(&next.nodes, delta); }
        if let Some(delta) = &self.edges { next.edges = apply_edges_delta(&next.edges, delta); }
        if let Some(meta) = &self.meta { next.meta = meta.clone(); }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(camera);
        take!(meta);
        take!(selected_ids);
        take!(active_utility_id);
        take!(camera_x);
        take!(camera_y);
        take!(camera_zoom);
        take!(selection_method);
        take!(grid_snap_enabled);
        take!(grid_factor);
        take!(suggestion_offset);
        take!(fill_count);
        take!(brush_candidate_index);
        take!(brush_candidate_source_handle_id);
        take!(locale);
        take!(terminology);
        take!(lod_mode_by_pane_json);
        take!(engagement_input_by_pane_json);
        take!(brush_candidates_json);
        take!(node_kind_weights_json);
        take!(handle_kind_weights_json);
        take!(active_utility_by_window_id_json);
        take!(hovered_node_id);
        take!(preview_seq);
        if let Some(delta) = other.nodes {
            match &mut self.nodes {
                Some(existing) => {
                    existing.removed.extend(delta.removed);
                    existing.added.extend(delta.added);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.nodes = Some(delta),
            }
        }
        if let Some(delta) = other.edges {
            match &mut self.edges {
                Some(existing) => {
                    existing.removed.extend(delta.removed);
                    existing.added.extend(delta.added);
                    existing.patched.extend(delta.patched);
                    if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                }
                None => self.edges = Some(delta),
            }
        }
    }
}
//#endregion 🔖️Apply

