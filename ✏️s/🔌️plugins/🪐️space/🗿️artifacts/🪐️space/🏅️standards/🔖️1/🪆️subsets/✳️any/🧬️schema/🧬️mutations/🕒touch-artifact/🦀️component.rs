//! 🕒 Direct SSpace mutation — `TouchArtifact` stamps an id-keyed row's `updatedAtMs`/`updatedBy` (the
//! auto-checkpoint hook per contract §C5 dispatches this after every checkpoint).
use crate::artifacts::space::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "touch-artifact")]
pub struct TouchArtifact {
    pub id: String,
    pub updated_at_ms: u64,
    pub updated_by: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn touch_artifact(id: String, updated_at_ms: u64, updated_by: String) -> SSpaceMutation {
    SSpaceMutation::TouchArtifact(TouchArtifact { id, updated_at_ms, updated_by })
}

impl protocol::MutationKind<SSpaceSnapshot, SSpaceMutation> for TouchArtifact {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "artifact", kind: "touch-artifact", record: "TouchedArtifact" };

    async fn diff(&self, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SSpaceSnapshot) -> Vec<SSpaceMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Touch artifact \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
