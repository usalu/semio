//! Puzzle2d mutation — `ChangeEdgeKind`: changes an edge's `edge_kind` catalog reference.
use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `change-edge-kind` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-edge-kind")]
pub struct ChangeEdgeKind {
    pub id: String,
    pub new_edge_kind: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_edge_kind(id: String, new_edge_kind: Option<String>) -> Puzzle2dMutation {
    Puzzle2dMutation::ChangeEdgeKind(ChangeEdgeKind { id, new_edge_kind })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ChangeEdgeKind {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "edge", kind: "change-edge-kind", record: "ChangedEdgeKind" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> Puzzle2dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change edge \"{}\" kind", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
