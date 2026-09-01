//! 🎈️️ `unflatten` — restores the node addressed by `at` wholesale to `original` (a captured
//! hierarchy, per `📓️taxonomy.md`'s `flatten`/`unflatten` row: "addr + captured hierarchy").

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{DrawNodeDiff, NodePath, SemioDrawingDiff, diff_at_path, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::{SemioDrawingMutation, flatten_node};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct UnflattenNode {
    pub at: NodePath,
    pub original: DrawNode,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for UnflattenNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "unflatten", entity: "node", kind: "unflatten-node", record: "UnflattenedNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Unflatten node in layer #{}", self.at.layer)
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload
