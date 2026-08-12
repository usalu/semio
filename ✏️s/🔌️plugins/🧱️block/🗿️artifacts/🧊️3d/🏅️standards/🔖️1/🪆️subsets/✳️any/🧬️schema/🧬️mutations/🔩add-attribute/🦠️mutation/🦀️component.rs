//! 🔩 Block3d mutation — `AddAttribute`: a free-form key/value attribute attachment.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;
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
pub fn add_attribute(attribute: BlockAttribute) -> Block3dMutation {
    Block3dMutation::AddAttribute(AddAttribute { attribute })
}

impl protocol::MutationKind<Block3dSnapshot, Block3dMutation> for AddAttribute {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "attribute", kind: "add-attribute", record: "AddedAttribute" };

    fn diff(&self, base: &Block3dSnapshot) -> Block3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Add attribute \"{}\"", self.attribute.key)
    }
    fn target(&self) -> Vec<String> {
        vec![self.attribute.key.clone()]
    }
}
//#endregion 🔖️Mutation
