//! 🏷️ `create-properties` — sets the kit's `properties` CHILD slot to a new owned `value` tree
//! handle (overwrite-aware — undo restores whichever handle was there before).

use crate::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateProperties {
    pub child_id: String,
    pub target: semio_framework_artifact_reference::ArtifactRef,
}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for CreateProperties {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "properties", kind: "create-properties", record: "CreatedProperties" };

    fn diff(&self, base: &SemioKitSnapshot) -> protocol::MutationOutcome<<SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Result<Vec<SemioKitMutation>, semio_framework_value::ValueError> {
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
