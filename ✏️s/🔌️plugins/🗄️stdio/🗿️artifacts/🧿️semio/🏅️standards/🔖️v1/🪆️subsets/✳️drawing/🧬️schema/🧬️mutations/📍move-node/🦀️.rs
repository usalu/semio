//! 📍️ `move-node` — absolute spatial reposition of the node addressed by `at` (SMO's approved
//! `move{node_id,new_origin}`, slug widened to `move-node` since `move` is a Rust keyword and
//! cannot name a module). Reaches `Group.transform.translation.{x,y}`, or `Text`/`Image.at`
//! directly -- `Path` has no origin field of its own (its geometry lives entirely in `segments`),
//! so this is honestly a no-op there rather than an approximation.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{NodePath, SemioDrawingDiff, diff_move_node, node_at, node_origin};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveNode {
    pub at: NodePath,
    pub new_origin: SemioPoint2,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for MoveNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node", kind: "move-node", record: "MovedNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Move node in layer #{}", self.at.layer)
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload
