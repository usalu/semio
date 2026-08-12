//! 🗑️ Draw mutation — `DeleteLayer`: removes an id-keyed layer (captures its full subtree +
//! location for undo).
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ `delete-layer` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-layer")]
pub struct DeleteLayer {
    pub layer_id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_layer(layer_id: String) -> DrawMutation {
    DrawMutation::DeleteLayer(DeleteLayer { layer_id })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for DeleteLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "layer", kind: "delete-layer", record: "DeletedLayer" };

    fn diff(&self, base: &DrawSnapshot) -> DrawDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete layer \"{}\"", self.layer_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
