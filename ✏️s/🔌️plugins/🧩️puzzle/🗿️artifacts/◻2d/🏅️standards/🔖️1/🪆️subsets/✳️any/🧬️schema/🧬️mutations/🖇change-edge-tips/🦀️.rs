//! Puzzle2d mutation — `ChangeEdgeTips`: changes an edge's source/target terminator markers together (one cohesive tips facet).

use crate::artifacts::puzzle2d::diff::Puzzle2dDiff;
use crate::artifacts::puzzle2d::mutations::Puzzle2dMutation;
use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `change-edge-tips` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-edge-tips")]
pub struct ChangeEdgeTips {
    pub id: String,
    pub new_source_tip: Option<String>,
    pub new_target_tip: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_edge_tips(id: String, new_source_tip: Option<String>, new_target_tip: Option<String>) -> Puzzle2dMutation {
    Puzzle2dMutation::ChangeEdgeTips(ChangeEdgeTips { id, new_source_tip, new_target_tip })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ChangeEdgeTips {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "edge", kind: "change-edge-tips", record: "ChangedEdgeTips" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change edge \"{}\" tips", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
