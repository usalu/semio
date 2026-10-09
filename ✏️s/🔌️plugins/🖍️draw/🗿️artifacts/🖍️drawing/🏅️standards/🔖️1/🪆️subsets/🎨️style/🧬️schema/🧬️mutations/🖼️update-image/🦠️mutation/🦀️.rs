//! 🖼️ Atomic image asset and size edits with preserved layer identity.
use crate::{DrawingSnapshot, DrawingMutation};
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-image")]
pub struct UpdateImage {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub image_key: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub width: f64,
    pub height: f64,
}
pub fn update_image(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, image_key: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, width: f64, height: f64) -> DrawingMutation {
    DrawingMutation::UpdateImage(UpdateImage { layer_id, image_key, width, height })
}
impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for UpdateImage {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "image", kind: "update-image", record: "UpdatedImage" };
    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Edit image", "Bild bearbeiten") }
    fn target(&self) -> Vec<String> { vec![self.layer_id.to_string_owner()] }
}
