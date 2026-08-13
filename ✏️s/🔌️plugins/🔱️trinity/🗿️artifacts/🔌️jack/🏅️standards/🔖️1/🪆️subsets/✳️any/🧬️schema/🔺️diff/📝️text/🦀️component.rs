//! 🔺️ Jack artifact — sparse field-delta diff codec and apply/absorb.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `apply_nodes_delta`/`apply_edges_delta`/
//! `apply_property_patch` and the whole `diff_nodes_*`/`diff_edges_*`/`diff_delete_node` builder set
//! are gone — `JackDiff.content: Option<JackContentChild>` is now a single whole-handle-replace slot
//! (matches `dag`'s/`writer`'s precedent). Every triad's own `🔺️diff` leaf now builds the new scene
//! itself (reading `jack_working_scene(base)`, applying its specific semantics to a clone) and calls
//! `diff_replace_content`.

use crate::artifacts::jack::schema::diff::JackDiff;
use crate::artifacts::jack::schema::JackArtifact;
use crate::artifacts::jack::{Edge, JackSnapshot, Node};
use protocol::MutationDiff;


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
        if let Some(content) = &self.content {
            next.content = content.clone();
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
        if let Some(content) = &self.content {
            next.content = content.clone();
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
        take!(content);
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
    }
}

/// 🏗️ The one builder every triad's `🔺️diff` leaf funnels through — mints+caches a fresh
/// content-addressed handle for the new `(nodes, edges)` scene and wraps it as a whole-handle-replace
/// sparse diff. Mirrors `dag`'s `diff_replace_content` precedent exactly.
pub fn diff_replace_content(nodes: Vec<Node>, edges: Vec<Edge>) -> JackDiff {
    JackDiff { content: Some(crate::artifacts::jack::jack_content_child_handle_and_cache(nodes, edges)), ..Default::default() }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jack_diff_absorb_merges_content() {
        let mut diff = JackDiff::default();
        let other = diff_replace_content(
            vec![Node {
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
            vec![],
        );
        diff.absorb(other.clone());
        assert_eq!(diff.content, other.content);
    }
}
//#endregion ️Tests
