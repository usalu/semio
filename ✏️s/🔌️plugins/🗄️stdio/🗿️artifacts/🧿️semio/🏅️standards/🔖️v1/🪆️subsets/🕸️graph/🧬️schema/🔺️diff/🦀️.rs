//! 🔺️ SemioGraphDiff — sparse per-field diff over `SemioGraphSnapshot`. `graph` has exactly two
//! mutable top-level fields (`nodes`/`edges`, both id-keyed sets with no user-meaningful display
//! order per `SEMANTIC-MUTATIONS-OVERHAUL`'s `📓️derivation-rules.md` rule 2), so the diff carries
//! two independent `Option<…List>` slots: whole-list-wrappers rebuilt POSITIONALLY from `base` by
//! each mutation triad's own `🔺️diff` leaf (never a generic `between()` re-derivation) — the same
//! shape `🔤️text`'s `SemioTextDiff`/`SemioTextRunList` uses for its own id-less `runs` collection.
//! No `snapshot: Option<SemioGraphSnapshot>` full-replace slot anywhere — whole-document replace is
//! `ArtifactStore::reset`, outside history.

use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot};
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️NodeList
/// 📋 Whole-list wrapper for the `nodes` field diff — every mutation triad rebuilds the full
/// `values` vec from `base` and wraps it here (`SemioTextRunList`'s own shape).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioGraphNodeList {
    pub values: Vec<SemioGraphNode>,
}
//#endregion 🔖️NodeList

//#region 🔖️EdgeList
/// 📋 Whole-list wrapper for the `edges` field diff — same shape as [`SemioGraphNodeList`].
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioGraphEdgeList {
    pub values: Vec<SemioGraphEdge>,
}
//#endregion 🔖️EdgeList

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.graph.diff")]
pub struct SemioGraphDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<SemioGraphNodeList>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edges: Option<SemioGraphEdgeList>,
}

impl SemioGraphDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.nodes.is_none() && self.edges.is_none()
    }
}

impl MutationDiff<SemioGraphSnapshot> for SemioGraphDiff {
    fn apply(&self, base: &SemioGraphSnapshot) -> protocol::MutationApplyResult<SemioGraphSnapshot> {
        let mut next = base.clone();
        if let Some(list) = &self.nodes {
            next.nodes = list.values.clone();
        }
        if let Some(list) = &self.edges {
            next.edges = list.values.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.nodes.is_some() {
            self.nodes = other.nodes;
        }
        if other.edges.is_some() {
            self.edges = other.edges;
        }
    }
}

/// 🧮️ `graph`'s own `DiffAlgebra` — required by the `✉️base` envelope's own dispatch (`SemioDiff`
/// delegates `between`/`inverse`/`is_empty` straight through to every wrapped subset's own impl).
/// Whole-list `between`/`inverse` are honest here (not apply-then-capture): `graph` has exactly two
/// mutable fields, so a change is fully described by "the new/old `nodes`/`edges` value", same
/// shape every mutation triad's own `🔺️diff` leaf already produces.
impl protocol::command::DiffAlgebra<SemioGraphSnapshot> for SemioGraphDiff {
    fn between(base: &SemioGraphSnapshot, other: &SemioGraphSnapshot) -> Self {
        SemioGraphDiff { nodes: (base.nodes != other.nodes).then(|| SemioGraphNodeList { values: other.nodes.clone() }), edges: (base.edges != other.edges).then(|| SemioGraphEdgeList { values: other.edges.clone() }) }
    }
    fn inverse(&self, base: &SemioGraphSnapshot) -> Self {
        SemioGraphDiff { nodes: self.nodes.as_ref().map(|_| SemioGraphNodeList { values: base.nodes.clone() }), edges: self.edges.as_ref().map(|_| SemioGraphEdgeList { values: base.edges.clone() }) }
    }
    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
















//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioGraphDiff` cases — single source of truth for `diff_grammar_conformance_
/// law`/`protocol_walk_law` in `🚪️io/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioGraphDiff> {
    use crate::standards::v1::subsets::graph::schema::snapshot::demo_graph_snapshot;
    let demo = demo_graph_snapshot();
    vec![
        SemioGraphDiff::default(),
        SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: demo.nodes.clone() }), edges: None },
        SemioGraphDiff { nodes: None, edges: Some(SemioGraphEdgeList { values: demo.edges.clone() }) },
        SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: demo.nodes }), edges: Some(SemioGraphEdgeList { values: demo.edges }) },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
