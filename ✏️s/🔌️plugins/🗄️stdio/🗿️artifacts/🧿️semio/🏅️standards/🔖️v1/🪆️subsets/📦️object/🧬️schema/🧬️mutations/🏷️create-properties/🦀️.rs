//! 🧱️ Creates an exact properties child reference in a vacant Object slot.

use crate::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(deny_unknown_fields)]
pub struct CreateProperties {
    pub child_id: String,
    pub target: semio_framework_artifact_reference::ArtifactRef,
}

impl<Op> protocol::MutationKind<SemioObjectSnapshot, Op> for CreateProperties
where
    Op: protocol::Mutation<SemioObjectSnapshot, Diff = crate::schema::diff::SemioObjectDiff> + From<CreateProperties> + From<crate::schema::mutations::delete_properties::DeleteProperties>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "properties", kind: "create-properties", record: "CreatedProperties" };

    fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioObjectSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create properties child {}", self.child_id), &format!("Eigenschaftenkind {} erstellen", self.child_id))
    }
    fn target(&self) -> Vec<String> {
        vec!["properties".to_string()]
    }
}
//#endregion 🔖️Payload
