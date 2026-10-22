//! 🏗️ `create-object` — appends a new owned CHILD handle to the kit's `objects` collection
//! (FINAL-state addressing per `📓️taxonomy.md`'s insert/create convention).

use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateObject {
    pub child_id: String,
    pub target: semio_framework_artifact_reference::ArtifactRef,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub at: Option<usize>,
}

impl<Op> protocol::MutationKind<SemioKitSnapshot, Op> for CreateObject
where
    Op: protocol::Mutation<SemioKitSnapshot, Diff = crate::schema::diff::SemioKitDiff> + From<CreateObject> + From<crate::schema::mutations::delete_object::DeleteObject>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "object", kind: "create-object", record: "CreatedObject" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create object child {}", self.child_id), &format!("Objektkind {} erstellen", self.child_id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.child_id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<SemioKitSnapshot, Op> for CreateObject
where
    Op: protocol::ColdMutation<SemioKitSnapshot, ReferenceDiff = crate::schema::diff::SemioKitDiff> + From<CreateObject> + From<crate::schema::mutations::delete_object::DeleteObject>,
{
    fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<Op as protocol::ColdMutation<SemioKitSnapshot>>::ReferenceDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Payload
