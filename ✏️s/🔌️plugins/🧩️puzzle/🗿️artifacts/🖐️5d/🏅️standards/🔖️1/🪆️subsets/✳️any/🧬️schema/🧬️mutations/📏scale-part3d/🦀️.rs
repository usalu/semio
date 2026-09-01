//! Puzzle5d mutation — `ScalePart3d`: changes a part's 3D-projection freeform scale.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `scale-part3d` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "scale-part3d")]
pub struct ScalePart3d {
    pub id: String,
    pub new_scale: Option<crate::artifacts::puzzle5d::Puzzle5dScale>,
}

impl protocol::MutationKind<Puzzle5dSnapshot, Puzzle5dMutation> for ScalePart3d {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "scale", entity: "part", kind: "scale-part3d", record: "ScaledPart3d" };

    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Scale part \"{}\" (3d)", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn scale_part_3d(id: String, new_scale: Option<crate::artifacts::puzzle5d::Puzzle5dScale>) -> Puzzle5dMutation {
    Puzzle5dMutation::ScalePart3d(ScalePart3d { id, new_scale })
}
