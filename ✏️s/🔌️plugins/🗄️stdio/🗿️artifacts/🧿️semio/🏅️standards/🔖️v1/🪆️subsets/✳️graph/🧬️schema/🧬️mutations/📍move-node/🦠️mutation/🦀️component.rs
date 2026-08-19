//! 📍 `move-node` — absolute spatial reposition of a node (taxonomy's `move` verb — FINAL-state
//! absolute position, not a relative offset).

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::{GraphNodeId, SemioGraphSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveNode {
    pub id: GraphNodeId,
    pub new_position: SemioPoint2,
}

impl protocol::MutationKind<SemioGraphSnapshot, SemioGraphMutation> for MoveNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "node", kind: "move-node", record: "MovedNode" };

    async fn diff(&self, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<<SemioGraphMutation as protocol::Mutation<SemioGraphSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Move node \"{}\" to ({}, {})", self.id.value, self.new_position.x, self.new_position.y)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.value.clone()]
    }
}
//#endregion 🔖️Payload
