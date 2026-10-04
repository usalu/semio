//! 🌱 Direct SSpace mutation — `CreateArtifact` brings a new id-keyed row into the space's artifact index.
use crate::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::{SSpaceSnapshot, SpaceArtifactRow};

//#region 🔖️Mutation
/// 🌱 `create-artifact` payload — the full initial row (id/name/kind/schema/dialect/timestamps all
/// fixed at creation, mirroring `dag`'s `CreateNode { node: DagNodeSpec }` shape).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-artifact")]
pub struct CreateArtifact {
    #[dsl(block)]
    pub artifact: SpaceArtifactRow,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_artifact(artifact: SpaceArtifactRow) -> SSpaceMutation {
    SSpaceMutation::CreateArtifact(CreateArtifact { artifact })
}

impl protocol::MutationKind<SSpaceSnapshot, SSpaceMutation> for CreateArtifact {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "artifact", kind: "create-artifact", record: "CreatedArtifact" };

    fn diff(&self, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SSpaceSnapshot) -> Result<Vec<SSpaceMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create artifact \"{}\"", self.artifact.id), &format!("Artefakt \"{}\" erstellen", self.artifact.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.artifact.id.clone()]
    }
}
//#endregion 🔖️Mutation
