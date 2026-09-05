//! 🏷️ `create-properties` — sets the object's `properties` CHILD slot to a new owned `value` tree
//! handle (overwrite-aware, same convention as `create-brep`/`create-mesh`).

use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::{SemioObjectMutation, delete_properties};
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateProperties {
    pub child_id: String,
    pub target: store::os_io::ArtifactRef,
}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for CreateProperties {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "properties", kind: "create-properties", record: "CreatedProperties" };

    fn diff(&self, base: &SemioObjectSnapshot) -> protocol::MutationOutcome<<SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create properties child {}", self.child_id)
    }
    fn target(&self) -> Vec<String> {
        vec!["properties".to_string()]
    }
}
//#endregion 🔖️Payload
