//! ➖ Block3d mutation — `RemoveRepresentationAttribute`: a member of a representation's nested `attributes`.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ➖ `remove-representation-attribute` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "remove-representation-attribute")]
pub struct RemoveRepresentationAttribute {
    pub id: String,
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_representation_attribute(id: String, key: String) -> Block3dMutation {
    Block3dMutation::RemoveRepresentationAttribute(RemoveRepresentationAttribute { id, key })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for RemoveRepresentationAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "representation-attribute", kind: "remove-representation-attribute", record: "RemovedRepresentationAttribute" };

    fn diff(&self, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Remove attribute \"{}\" from representation \"{}\"", self.key, self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
