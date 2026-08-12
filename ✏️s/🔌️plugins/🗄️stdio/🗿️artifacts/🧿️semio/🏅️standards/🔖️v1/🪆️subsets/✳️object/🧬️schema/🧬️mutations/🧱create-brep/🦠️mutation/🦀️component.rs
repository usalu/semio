//! 🧱️ `create-brep` — sets the object's `brep` CHILD slot to a new owned handle. If the slot was
//! already occupied, this OVERWRITES it (the inverse restores whichever handle was there before,
//! not merely "delete" — see `↩️inverse`).

use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;
use crate::artifacts::semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateBrep {
    pub child_id: String,
    pub target: store::os_io::ArtifactRef,
}

impl protocol::MutationKind<SemioObjectSnapshot, SemioObjectMutation> for CreateBrep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "brep", kind: "create-brep", record: "CreatedBrep" };

    fn diff(&self, base: &SemioObjectSnapshot) -> <SemioObjectMutation as protocol::Mutation<SemioObjectSnapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &SemioObjectSnapshot) -> Vec<SemioObjectMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create brep child {}", self.child_id)
    }
    fn target(&self) -> Vec<String> {
        vec!["brep".to_string()]
    }
}
//#endregion 🔖️Payload
