//! 🧩️ Block2d mutation — `AddAttribute`: a free-form key/value attribute attachment.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::{BlockAttribute};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🧩️ `add-attribute` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "add-attribute")]
pub struct AddAttribute {
    #[dsl(block)]
    pub attribute: BlockAttribute,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn add_attribute(attribute: BlockAttribute) -> Block2dMutation {
    Block2dMutation::AddAttribute(AddAttribute { attribute })
}

impl protocol::MutationKind<Block2dSnapshot, Block2dMutation> for AddAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "attribute", kind: "add-attribute", record: "AddedAttribute" };

    async fn diff(&self, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
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
