//! 🗑️ SSpace mutation — `DeleteArtifact`: removes an id-keyed row from the space's artifact index.
use crate::artifacts::space::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-artifact")]
pub struct DeleteArtifact {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_artifact(id: String) -> SSpaceMutation {
    SSpaceMutation::DeleteArtifact(DeleteArtifact { id })
}

impl protocol::MutationKind<SSpaceSnapshot, SSpaceMutation> for DeleteArtifact {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "artifact", kind: "delete-artifact", record: "DeletedArtifact" };

    fn diff(&self, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SSpaceSnapshot) -> Vec<SSpaceMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete artifact \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
