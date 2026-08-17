//! 🔺️ DAG artifact — sparse field-delta diff codec and apply/absorb.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: the collection-delta appliers
//! (`apply_nodes_delta`/`apply_edges_delta`/`apply_identified_delta`/`absorb_nodes_delta`/
//! `absorb_edges_delta`) are gone — `content` is opaque, so `apply`/`absorb` both collapse to a
//! single whole-handle-replace branch, the same pattern flow's `FlowDiff::apply`/`absorb` and
//! writer's `WriterDiff::apply`/`absorb` already established.

use crate::artifacts::dag::schema::DagArtifact;
use crate::artifacts::dag::{DagContentChild, DagFixtureEdge, DagNodeSpec, DagSnapshot};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

pub use crate::artifacts::dag::schema::diff::*;

//#region 🔖️ReplaceContent
/// 🏗️ Every mutation triad's `🔺️diff` builder goes through this: read the current scene off `base`
/// via `crate::artifacts::dag::dag_working_scene`, apply its own specific semantics to a clone of
/// that scene, then mint+cache a whole new content handle here — the "mint+cache whole handle, never
/// apply-then-capture" pattern flow's `diff_replace_content`/writer's `diff_set_text` established.
pub fn diff_replace_content(nodes: Vec<DagNodeSpec>, edges: Vec<DagFixtureEdge>) -> DagDiff {
    DagDiff { content: Some(crate::artifacts::dag::dag_content_child_handle_and_cache(nodes, edges)), ..Default::default() }
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
            if let Some(value) = &self.locale {
                next.locale = value.clone();
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
        take!(locale);
    }
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::dag::default_snapshot;
    use crate::artifacts::dag::schema::mutations::delete_node;
    use protocol::Mutation;

    #[test]
    fn dag_diff_default_has_no_pending_writes() {
        let diff = DagDiff::default();
        assert!(diff.content.is_none());
    }

    #[test]
    fn delete_node_diff_removes_the_node() {
        let base = default_snapshot();
        let id = base.nodes().first().expect("fixture has a node").id.clone();
        let mutation = delete_node(id.clone());
        let outcome = mutation.diff(&base);
        assert!(outcome.diff().apply(&base).expect("valid mutation diff").nodes().iter().all(|node| node.id != id));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_grammar_conformance {
    use super::*;

    #[test]
    fn component_grammar_semio_is_grammar_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Grammar);
        assert!(!COMPONENT_GRAMMAR_SEMIO.is_empty());
        let _ = COMPONENT_GRAMMAR_PATH;
    }
}
