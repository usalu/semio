//! 🧩 Block5d mutation — `AddRepresentationAttribute`: a member of a representation's nested `attributes`.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockAttribute};
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
pub fn add_representation_attribute(id: String, attribute: BlockAttribute) -> Block5dMutation {
    Block5dMutation::AddRepresentationAttribute(AddRepresentationAttribute { id, attribute })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for AddRepresentationAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "representation-attribute", kind: "add-representation-attribute", record: "AddedRepresentationAttribute" };

    fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add attribute \"{}\" to representation \"{}\"", self.attribute.key, self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
