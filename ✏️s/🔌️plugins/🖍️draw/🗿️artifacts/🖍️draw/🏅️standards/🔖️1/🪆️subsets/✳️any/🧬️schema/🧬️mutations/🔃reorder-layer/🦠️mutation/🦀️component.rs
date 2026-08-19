//! 🔃 Draw mutation — `ReorderLayer`: repositions (and optionally re-parents) an existing layer to
//! a FINAL-state `(parent_id, index)` address — never spatial.
use crate::artifacts::draw::diff::DrawDiff;
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔃 `reorder-layer` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "reorder-layer")]
pub struct ReorderLayer {
    pub layer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    pub index: usize,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn reorder_layer(layer_id: String, parent_id: Option<String>, index: usize) -> DrawMutation {
    DrawMutation::ReorderLayer(ReorderLayer { layer_id, parent_id, index })
}

impl protocol::MutationKind<DrawSnapshot, DrawMutation> for ReorderLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "layer", kind: "reorder-layer", record: "ReorderedLayer" };

    async fn diff(&self, base: &DrawSnapshot) -> protocol::MutationOutcome<DrawDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &DrawSnapshot) -> Vec<DrawMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Reorder layer \"{}\"", self.layer_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
