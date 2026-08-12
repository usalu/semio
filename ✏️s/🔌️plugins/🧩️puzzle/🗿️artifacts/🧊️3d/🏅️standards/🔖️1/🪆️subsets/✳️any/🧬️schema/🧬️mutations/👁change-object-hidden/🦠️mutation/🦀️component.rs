//! Puzzle3d mutation — `ChangeObjectHidden`: changes an object's hidden flag.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `change-object-hidden` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "change-object-hidden")]
pub struct ChangeObjectHidden {
    pub id: String,
    pub new_hidden: bool,
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for ChangeObjectHidden {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "object", kind: "change-object-hidden", record: "ChangedObjectHidden" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> Puzzle3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change object \"{}\" hidden", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_object_hidden(id: String, new_hidden: bool) -> Puzzle3dMutation {
    Puzzle3dMutation::ChangeObjectHidden(ChangeObjectHidden { id, new_hidden })
}
