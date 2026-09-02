//! 🧱 Block5d mutation — `CreateRepresentation`: a new representation (mesh at a LOD/tag combination).

use crate::BlockRepresentation;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dRepresentationsDelta};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Mutation
/// 🧱 `create-representation` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "create-representation")]
pub struct CreateRepresentation {
    #[dsl(block)]
    pub representation: BlockRepresentation,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn create_representation(representation: BlockRepresentation) -> Block5dMutation {
    Block5dMutation::CreateRepresentation(CreateRepresentation { representation })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for CreateRepresentation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "representation", kind: "create-representation", record: "CreatedRepresentation" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create representation \"{}\"", self.representation.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.representation.id.clone()]
    }
}
//#endregion 🔖️Mutation
