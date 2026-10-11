//! 🕒 Direct SSpace mutation — `TouchArtifact` stamps an id-keyed row's `updatedAtMs`/`updatedBy` (the
//! auto-checkpoint hook per contract §C5 dispatches this after every checkpoint).
use crate::standards::v1::subsets::any::schema::diff::SSpaceDiff;
use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "touch-artifact")]
pub struct TouchArtifact {
    pub id: String,
    pub updated_at_ms: u64,
    pub updated_by: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn touch_artifact(id: String, updated_at_ms: u64, updated_by: String) -> SSpaceMutation {
    SSpaceMutation::TouchArtifact(TouchArtifact { id, updated_at_ms, updated_by })
}

impl protocol::MutationKind<SSpaceSnapshot, SSpaceMutation> for TouchArtifact {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "artifact", kind: "touch-artifact", record: "TouchedArtifact" };

    fn diff(&self, base: &SSpaceSnapshot) -> protocol::MutationOutcome<SSpaceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SSpaceSnapshot) -> Result<Vec<SSpaceMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Touch artifact \"{}\"", self.id), &format!("Artefakt \"{}\" als geändert markieren", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
