//! 📝️ Atomic text content and size edits with preserved layer identity.
use crate::{DrawingSnapshot, DrawingMutation};
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-text")]
pub struct UpdateText {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub content: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub size: f64,
    pub font_family:crate::DrawingFontFamily,
}
pub fn update_text(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, content: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, size: f64,font_family:crate::DrawingFontFamily) -> DrawingMutation {
    DrawingMutation::UpdateText(UpdateText { layer_id, content, size,font_family })
}
impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for UpdateText {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "text", kind: "update-text", record: "UpdatedText" };
    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Edit text", "Text bearbeiten") }
    fn target(&self) -> Vec<String> { vec![self.layer_id.to_string_owner()] }
}
