//! 🔺️ Jack artifact — sparse field-delta diff codec and apply/absorb.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: `apply_nodes_delta`/`apply_edges_delta`/
//! `apply_property_patch` and the whole `diff_nodes_*`/`diff_edges_*`/`diff_delete_node` builder set
//! are gone — `JackDiff.content: Option<JackContentChild>` is now a single whole-handle-replace slot
//! (matches `dag`'s/`writer`'s precedent). Every triad's own `🔺️diff` leaf now builds the new scene
//! itself (reading `jack_working_scene(base)`, applying its specific semantics to a clone) and calls
//! `diff_replace_content`.

use crate::standards::v1::subsets::any::schema::diff::JackDiff;
use crate::standards::v1::subsets::any::schema::JackArtifact;
use crate::{Edge, JackSnapshot, Node};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
impl JackDiff {
    /// 🧬️ Applies document-owned sparse entries onto the artifact.
    pub fn apply_to_artifact(&self, artifact: &JackArtifact) -> protocol::MutationApplyResult<JackArtifact> {
        Ok({
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
            next
        })
    }
}

impl MutationDiff<JackSnapshot> for JackDiff {
    fn apply(&self, snapshot: &JackSnapshot) -> protocol::MutationApplyResult<JackSnapshot> {
        Ok({
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
        })
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
    }
}

/// 🏗️ The one builder every triad's `🔺️diff` leaf funnels through — mints+caches a fresh
/// content-addressed handle for the new `(nodes, edges)` scene and wraps it as a whole-handle-replace
/// sparse diff. Mirrors `dag`'s `diff_replace_content` precedent exactly.
pub fn diff_replace_content(nodes: Vec<Node>, edges: Vec<Edge>) -> JackDiff {
    JackDiff { content: Some(crate::jack_content_child_with_owner(nodes, edges)), ..Default::default() }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion ️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type JackDiffText = String;
//#endregion 🚚️Carrier
