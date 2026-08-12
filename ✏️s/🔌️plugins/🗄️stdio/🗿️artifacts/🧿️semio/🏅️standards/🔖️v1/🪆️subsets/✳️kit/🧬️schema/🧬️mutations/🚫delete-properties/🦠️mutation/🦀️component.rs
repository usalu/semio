//! 🚫️ `delete-properties` — clears the kit's `properties` CHILD slot. Idempotent; inverse
//! escrows from BASE.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteProperties {}

impl protocol::MutationKind<SemioKitSnapshot, SemioKitMutation> for DeleteProperties {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "properties", kind: "delete-properties", record: "DeletedProperties" };

    fn diff(&self, base: &SemioKitSnapshot) -> <SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Vec<SemioKitMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Delete properties child".to_string()
    }
    fn target(&self) -> Vec<String> {
        vec!["properties".to_string()]
    }
}
//#endregion 🔖️Payload
