//! 🔄️ `rotate` — sets a `Group` node's `transform.rotation` (SMO-approved domain spatial
//! transform). Only `Group` carries a rotation field -- `Path`/`Text`/`Image` are honestly a
//! no-op, matching `move-node`'s own reach limits.

use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioQuaternion;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{NodePath, SemioDrawingDiff, diff_rotate_node, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RotateNode {
    pub at: NodePath,
    pub new_rotation: SemioQuaternion,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for RotateNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "node", kind: "rotate-node", record: "RotatedNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("RotateNode node in layer #{}", self.at.layer)
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload
