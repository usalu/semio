//! 🖐️️ `drag-nodes` — a SEPARATE plural mutation (never a bare `Vec` arg bolted onto the singular
//! `move-node`): relative spatial offset applied to every node in `ats` independently. Same
//! origin-field reach as `move-node` (`Group.transform.translation`, `Text`/`Image.at`; `Path`
//! is a no-op per-node).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DragNodes {
    pub ats: Vec<NodePath>,
    pub offset: SemioPoint2,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for DragNodes {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "drag", entity: "nodes", kind: "drag-nodes", record: "DraggedNodes" };

    async fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base).await
    }
    async fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base).await
    }
    async fn label(&self) -> String {
        format!("Drag {} node(s)", self.ats.len())
    }
    async fn target(&self) -> Vec<String> {
        self.ats.iter().map(|a| a.layer.to_string()).collect()
    }
}
//#endregion 🔖️Payload
