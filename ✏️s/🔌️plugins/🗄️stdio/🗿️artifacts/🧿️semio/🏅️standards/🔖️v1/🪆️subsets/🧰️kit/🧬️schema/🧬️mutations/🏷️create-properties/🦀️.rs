//! 🏷️ `create-properties` — sets the kit's `properties` CHILD slot to a new owned `value` tree
//! handle (overwrite-aware — undo restores whichever handle was there before).

use crate::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateProperties {
    pub child_id: String,
    pub target: semio_framework_artifact_reference::ArtifactRef,
}

impl<Op> protocol::MutationKind<SemioKitSnapshot, Op> for CreateProperties
where
    Op: protocol::Mutation<SemioKitSnapshot, Diff = crate::schema::diff::SemioKitDiff> + From<CreateProperties> + From<crate::schema::mutations::delete_properties::DeleteProperties>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "properties", kind: "create-properties", record: "CreatedProperties" };

    fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<Op as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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
