//! 🔩 Block5d mutation — `AddAttribute`: a free-form key/value attribute attachment.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockAttribute};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔩 `add-attribute` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-attribute")]
pub struct AddAttribute {
    #[dsl(block)]
    pub attribute: BlockAttribute,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn add_attribute(attribute: BlockAttribute) -> Block5dMutation {
    Block5dMutation::AddAttribute(AddAttribute { attribute })
}

impl protocol::MutationKind<Block5dSnapshot, Block5dMutation> for AddAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "attribute", kind: "add-attribute", record: "AddedAttribute" };

    async fn diff(&self, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Add attribute \"{}\"", self.attribute.key)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.attribute.key.clone()]
    }
}
//#endregion 🔖️Mutation
