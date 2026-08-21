//! 🧩 Block3d mutation — `AddRepresentationAttribute`: a member of a representation's nested `attributes`.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use crate::BlockAttribute;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧩 `add-representation-attribute` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-representation-attribute")]
pub struct AddRepresentationAttribute {
    pub id: String,
    #[dsl(block)]
    pub attribute: BlockAttribute,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn add_representation_attribute(id: String, attribute: BlockAttribute) -> Block3dMutation {
    Block3dMutation::AddRepresentationAttribute(AddRepresentationAttribute { id, attribute })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for AddRepresentationAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "representation-attribute", kind: "add-representation-attribute", record: "AddedRepresentationAttribute" };

    async fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Add attribute \"{}\" to representation \"{}\"", self.attribute.key, self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
