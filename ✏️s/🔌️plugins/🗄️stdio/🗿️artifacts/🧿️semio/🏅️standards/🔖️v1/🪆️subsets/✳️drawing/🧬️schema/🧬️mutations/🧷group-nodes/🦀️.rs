//! 🧷️ `group` — introduces a new `Group` node wrapping a CONTIGUOUS run of siblings (`indices`,
//! ascending, substituting for the taxonomy's "member ids" -- `DrawNode` has no stable id, the
//! same structural substitute `NodePath` already establishes throughout this facet) under the
//! `Group` addressed by `parent`. Restricted to contiguous runs so `ungroup` can restore the
//! EXACT original membership/positions losslessly (a non-contiguous grouping would interleave
//! with untouched siblings in a way `ungroup` could not reconstruct from `base` alone).

use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawGroupDiff, DrawNodeDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{SemioDrawingMutation, ungroup_node};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct GroupNodes {
    pub parent: NodePath,
    pub indices: Vec<usize>,
    pub transform: SemioTransform,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for GroupNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "group", entity: "nodes", kind: "group-nodes", record: "GroupedNodes" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Group {} node(s) in layer #{}", self.indices.len(), self.parent.layer)
    }
    fn target(&self) -> Vec<String> {
        vec![self.parent.layer.to_string()]
    }
}
//#endregion 🔖️Payload

//#region 🔖️ContiguousCheck
/// ✅️️ `true` iff `indices` is non-empty, strictly ascending, and every consecutive pair differs by
/// exactly 1 (a contiguous run) -- shared by this triad's `diff` and `↩️inverse/🦀️.rs`'s
/// own reverse construction (via `ungroup`, which always emits a genuinely contiguous run).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn is_contiguous_ascending(indices: &[usize]) -> bool {
    !indices.is_empty() && indices.windows(2).all(|w| w[1] == w[0] + 1)
}
//#endregion 🔖️ContiguousCheck
