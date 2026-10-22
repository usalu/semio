//! 🧱️ Creates an exact brep child reference in a vacant Object slot.

use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(deny_unknown_fields)]
pub struct CreateBrep {
    pub child_id: String,
    pub target: semio_framework_artifact_reference::ArtifactRef,
}

impl<Op> protocol::MutationKind<SemioObjectSnapshot, Op> for CreateBrep
where
    Op: protocol::Mutation<SemioObjectSnapshot, Diff = crate::schema::diff::SemioObjectDiff> + From<CreateBrep> + From<crate::schema::mutations::delete_brep::DeleteBrep>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "brep", kind: "create-brep", record: "CreatedBrep" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create brep child {}", self.child_id), &format!("B-Rep-Kind {} erstellen", self.child_id))
    }
    fn target(&self) -> Vec<String> {
        vec!["brep".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioObjectSnapshot, Op> for CreateBrep
where
    Op: protocol::ColdMutation<SemioObjectSnapshot, ReferenceDiff = crate::schema::diff::SemioObjectDiff> + From<CreateBrep> + From<crate::schema::mutations::delete_brep::DeleteBrep>,
{
    fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioObjectSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
