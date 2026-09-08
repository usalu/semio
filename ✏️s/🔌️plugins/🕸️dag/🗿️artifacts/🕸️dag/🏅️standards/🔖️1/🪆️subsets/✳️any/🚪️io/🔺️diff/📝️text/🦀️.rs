//! 🔺️ DAG artifact — sparse field-delta diff codec and apply/absorb.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: the collection-delta appliers
//! (`apply_nodes_delta`/`apply_edges_delta`/`apply_identified_delta`/`absorb_nodes_delta`/
//! `absorb_edges_delta`) are gone — `content` is opaque, so `apply`/`absorb` both collapse to a
//! single whole-handle-replace branch, the same pattern flow's `FlowDiff::apply`/`absorb` and
//! writer's `WriterDiff::apply`/`absorb` already established.

use crate::schema::DagArtifact;
use crate::{DagFixtureEdge, DagNodeSpec, DagSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::schema::diff::*;

//#region 🔖️ReplaceContent
/// 🏗️ Every mutation triad's `🔺️diff` builder goes through this: read the current scene off `base`
/// via `crate::dag_working_scene`, apply its own specific semantics to a clone of
/// that scene, then mint+cache a whole new content handle here — the "mint+cache whole handle, never
/// apply-then-capture" pattern flow's `diff_replace_content`/writer's `diff_set_text` established.
pub fn diff_replace_content(nodes: Vec<DagNodeSpec>, edges: Vec<DagFixtureEdge>) -> DagDiff {
    DagDiff { content: Some(crate::dag_content_child_with_owner(nodes, edges)), ..Default::default() }
}
//#endregion 🔖️ReplaceContent

//#region 🔖️Apply
impl DagDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &DagArtifact) -> protocol::MutationApplyResult<DagArtifact> {
        Ok({
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(content) = &self.content {
                next.content = content.clone();
            }
            if let Some(list) = &self.selected_node_ids {
                next.selected_node_ids = list.values.clone();
            }
            if let Some(value) = &self.camera {
                next.camera = value.clone();
            }
            next
        })
    }
}

impl MutationDiff<DagSnapshot> for DagDiff {
    fn apply(&self, snapshot: &DagSnapshot) -> protocol::MutationApplyResult<DagSnapshot> {
        Ok({
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(content) = &self.content {
                next.content = content.clone();
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
        take!(content);
        take!(selected_node_ids);
        take!(camera);
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semio-grammar-conformance/🦀️.rs"]
mod semio_grammar_conformance;
