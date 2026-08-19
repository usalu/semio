//! 🏔 Block5d mutation — `ChangeRepresentationLod`: a representation's `lod`.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏔 `change-representation-lod` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-representation-lod")]
pub struct ChangeRepresentationLod {
    pub id: String,
    pub new_lod: Option<String>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn change_representation_lod(id: String, new_lod: Option<String>) -> Block5dMutation {
    Block5dMutation::ChangeRepresentationLod(ChangeRepresentationLod { id, new_lod })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for ChangeRepresentationLod {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "representation", kind: "change-representation-lod", record: "ChangedRepresentationLod" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change representation \"{}\" LOD", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
