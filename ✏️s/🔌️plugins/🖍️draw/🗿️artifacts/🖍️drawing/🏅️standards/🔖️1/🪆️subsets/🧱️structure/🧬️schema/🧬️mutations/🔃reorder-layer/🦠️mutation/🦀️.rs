//! 🔃 Drawing mutation — `ReorderLayer`: repositions (and optionally re-parents) an existing layer to
//! a FINAL-state `(parent_id, index)` address — never spatial.
use crate::artifacts::drawing::diff::DrawingDiff;
use crate::artifacts::drawing::mutations::DrawingMutation;
use crate::artifacts::drawing::DrawingSnapshot;

//#region 🔖️Mutation
/// 🔃 `reorder-layer` payload.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "reorder-layer")]
pub struct ReorderLayer {
    pub layer_id: String,
    #[value(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub parent_id: Option<String>,
    pub index: usize,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn reorder_layer(layer_id: String, parent_id: Option<String>, index: usize) -> DrawingMutation {
    DrawingMutation::ReorderLayer(ReorderLayer { layer_id, parent_id, index })
}

impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for ReorderLayer {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "layer", kind: "reorder-layer", record: "ReorderedLayer" };

    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<DrawingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Reorder layer \"{}\"", self.layer_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.layer_id.clone()]
    }
}
//#endregion 🔖️Mutation
