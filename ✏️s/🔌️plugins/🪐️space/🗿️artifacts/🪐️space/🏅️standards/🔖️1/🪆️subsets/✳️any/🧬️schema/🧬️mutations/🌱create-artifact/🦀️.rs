//! 🌱 Direct SSpace mutation — `CreateArtifact` brings a new id-keyed row into the space's artifact index.
use crate::artifacts::space::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::{SSpaceSnapshot, SpaceArtifactRow};

//#region 🔖️Mutation
/// 🌱 `create-artifact` payload — the full initial row (id/name/kind/schema/dialect/timestamps all
/// fixed at creation, mirroring `dag`'s `CreateNode { node: DagNodeSpec }` shape).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-artifact")]
pub struct CreateArtifact {
    #[dsl(block)]
    pub artifact: SpaceArtifactRow,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_artifact(artifact: SpaceArtifactRow) -> SSpaceMutation {
    SSpaceMutation::CreateArtifact(CreateArtifact { artifact })
}

impl protocol::MutationKind<SSpaceSnapshot, SSpaceMutation> for CreateArtifact {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "artifact", kind: "create-artifact", record: "CreatedArtifact" };

    async fn diff(&self, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &SSpaceSnapshot) -> Vec<SSpaceMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create artifact \"{}\"", self.artifact.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.artifact.id.clone()]
    }
}
//#endregion 🔖️Mutation
