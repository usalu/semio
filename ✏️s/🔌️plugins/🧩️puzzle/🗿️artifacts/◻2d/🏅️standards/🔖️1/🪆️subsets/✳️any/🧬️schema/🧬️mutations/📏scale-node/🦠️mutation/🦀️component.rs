//! 📏️ Puzzle2d mutation — `ScaleNode`: applies a uniform scale factor to a node.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 📏️ `scale-node` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "scale-node")]
pub struct ScaleNode {
    pub id: String,
    pub new_scale: Option<f64>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn scale_node(id: String, new_scale: Option<f64>) -> Puzzle2dMutation {
    Puzzle2dMutation::ScaleNode(ScaleNode { id, new_scale })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ScaleNode {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "node", kind: "scale-node", record: "ScaledNode" };

    async fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Scale node \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
