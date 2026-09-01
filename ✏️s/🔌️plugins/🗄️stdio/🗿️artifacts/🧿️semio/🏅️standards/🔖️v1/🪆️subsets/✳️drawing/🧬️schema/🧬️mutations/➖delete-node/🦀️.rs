//! ➖️ `delete-node` — removes the `DrawNode` addressed BY `at` (whose last path segment is its
//! own index within its parent's `children`) from the recursive scene graph, capturing it for
//! `inverse`. `at.path` must be non-empty — a layer's own root cannot be deleted this way (that is
//! `delete-layer`'s job).

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawGroupDiff, DrawNodeDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{SemioDrawingMutation, create_node};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteNode {
    pub at: NodePath,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for DeleteNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete node in layer #{}", self.at.layer)
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload

//#region 🔖️ParentSplit
/// ✂️️ Splits `at` into (parent address, own index within the parent's `children`) -- `None` for
/// the layer root (empty `path`), which has no parent to remove a child from.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parent_and_index(at: &NodePath) -> Option<(NodePath, usize)> {
    let mut parent_path = at.path.clone();
    let index = parent_path.pop()?;
    Some((NodePath { layer: at.layer, path: parent_path }, index))
}
//#endregion 🔖️ParentSplit
