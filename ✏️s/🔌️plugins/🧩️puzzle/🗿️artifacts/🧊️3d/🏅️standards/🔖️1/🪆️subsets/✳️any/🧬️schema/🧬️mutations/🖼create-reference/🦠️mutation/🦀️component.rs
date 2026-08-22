//! Puzzle3d mutation — `CreateReference`: brings a new id-keyed reference into existence.
use crate::artifacts::puzzle3d::diff::Puzzle3dDiff;
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::{Puzzle3dReference, Puzzle3dSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// `create-reference` payload — full initial payload at an optional FINAL-state `index` (`None` appends). A
/// duplicate `reference.id` is a no-op.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-reference")]
pub struct CreateReference {
    #[dsl(block)]
    pub reference: Puzzle3dReference,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_reference(reference: Puzzle3dReference, index: Option<usize>) -> Puzzle3dMutation {
    Puzzle3dMutation::CreateReference(CreateReference { reference, index })
}

impl protocol::MutationKind<Puzzle3dSnapshot, Puzzle3dMutation> for CreateReference {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "reference", kind: "create-reference", record: "CreatedReference" };

    fn diff(&self, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create reference \"{}\"", self.reference.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.reference.id.clone()]
    }
}
//#endregion 🔖️Mutation
