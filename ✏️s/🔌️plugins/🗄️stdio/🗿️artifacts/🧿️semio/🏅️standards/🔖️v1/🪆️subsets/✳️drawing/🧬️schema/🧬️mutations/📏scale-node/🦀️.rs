//! 📏️ `scale` — sets a `Group` node's `transform.scale` (SMO-approved domain spatial transform).
//! Only `Group` carries a scale field -- every other node kind is honestly a no-op.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::{NodePath, SemioDrawingDiff, diff_scale_node, node_at};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::{DrawNode, SemioDrawingSnapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ScaleNode {
    pub at: NodePath,
    pub new_scale: SemioPoint3,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for ScaleNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "node", kind: "scale-node", record: "ScaledNode" };

    fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("ScaleNode node in layer #{}", self.at.layer)
    }
    fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload
