//! 🧱 Block5d mutation — `CreateRepresentation`: a new representation (mesh at a LOD/tag combination).
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockRepresentation};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧱 `create-representation` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-representation")]
pub struct CreateRepresentation {
    #[dsl(block)]
    pub representation: BlockRepresentation,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn create_representation(representation: BlockRepresentation) -> Block5dMutation {
    Block5dMutation::CreateRepresentation(CreateRepresentation { representation })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for CreateRepresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "representation", kind: "create-representation", record: "CreatedRepresentation" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create representation \"{}\"", self.representation.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.representation.id.clone()]
    }
}
//#endregion 🔖️Mutation
