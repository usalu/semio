//! 💫️ `ungroup` — dissolves the `Group` node addressed by `at`, splicing its children back into
//! its parent's `children` at its own position, in their existing relative order.

use crate::artifacts::semio::standards::v1::subsets::drawing::schema::diff::NodePath;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation;
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UngroupNode {
    pub at: NodePath,
}

impl protocol::MutationKind<SemioDrawingSnapshot, SemioDrawingMutation> for UngroupNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "ungroup", entity: "node", kind: "ungroup", record: "UngroupedNode" };

    async fn diff(&self, base: &SemioDrawingSnapshot) -> protocol::MutationOutcome<<SemioDrawingMutation as protocol::Mutation<SemioDrawingSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioDrawingSnapshot) -> Vec<SemioDrawingMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Ungroup node in layer #{}", self.at.layer)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.at.layer.to_string()]
    }
}
//#endregion 🔖️Payload
