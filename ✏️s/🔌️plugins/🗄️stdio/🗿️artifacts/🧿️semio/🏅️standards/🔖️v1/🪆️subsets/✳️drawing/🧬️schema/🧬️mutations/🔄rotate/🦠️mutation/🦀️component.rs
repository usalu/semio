//! 🔄️ `rotate` — sets a `Group` node's `transform.rotation` (SMO-approved domain spatial
//! transform). Only `Group` carries a rotation field -- `Path`/`Text`/`Image` are honestly a
//! no-op, matching `move-node`'s own reach limits.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioQuaternion;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rotate {
    pub at: NodePath,
    pub new_rotation: SemioQuaternion,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for Rotate {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rotate", entity: "node", kind: "rotate", record: "RotatedNode" };

    async fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rotate node in layer #{}", self.at.layer)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload
