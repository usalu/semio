//! Puzzle3d mutation — `ReplaceReferenceSource`: whole-value swap of a reference's media source (url + media kind together).
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `replace-reference-source` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "replace-reference-source")]
pub struct ReplaceReferenceSource {
    pub id: String,
    pub new_source: crate::artifacts::puzzle3d::Puzzle3dReferenceSource,
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ReplaceReferenceSource {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "reference", kind: "replace-reference-source", record: "ReplacedReferenceSource" };

    async fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Replace reference \"{}\" source", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn replace_reference_source(id: String, new_source: crate::artifacts::puzzle3d::Puzzle3dReferenceSource) -> Puzzle3dMutation {
    Puzzle3dMutation::ReplaceReferenceSource(ReplaceReferenceSource { id, new_source })
}
