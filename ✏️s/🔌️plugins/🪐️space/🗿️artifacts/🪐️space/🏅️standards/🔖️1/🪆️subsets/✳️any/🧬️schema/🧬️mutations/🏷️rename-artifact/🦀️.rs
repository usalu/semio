//! 🏷️ Direct SSpace mutation — `RenameArtifact` changes an id-keyed row's display name.
use crate::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-artifact")]
pub struct RenameArtifact {
    pub id: String,
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn rename_artifact(id: String, new_name: String) -> SSpaceMutation {
    SSpaceMutation::RenameArtifact(RenameArtifact { id, new_name })
}

impl protocol::MutationKind<SSpaceSnapshot, SSpaceMutation> for RenameArtifact {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "rename", entity: "artifact", kind: "rename-artifact", record: "RenamedArtifact" };

    fn diff(&self, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SSpaceSnapshot) -> Result<Vec<SSpaceMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Rename artifact \"{}\" to \"{}\"", self.id, self.new_name), &format!("Artefakt \"{}\" in \"{}\" umbenennen", self.id, self.new_name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
