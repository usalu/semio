//! 📝️ Atomic text content and size edits with preserved layer identity.
use crate::{DrawingSnapshot, DrawingMutation};
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-text")]
pub struct UpdateText {
    pub layer_id: String,
    pub content: String,
    pub size: f64,
}
pub fn update_text(layer_id: String, content: String, size: f64) -> DrawingMutation {
    DrawingMutation::UpdateText(UpdateText { layer_id, content, size })
}
impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for UpdateText {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "text", kind: "update-text", record: "UpdatedText" };
    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("Edit text", "Text bearbeiten") }
    fn target(&self) -> Vec<String> { vec![self.layer_id.clone()] }
}
