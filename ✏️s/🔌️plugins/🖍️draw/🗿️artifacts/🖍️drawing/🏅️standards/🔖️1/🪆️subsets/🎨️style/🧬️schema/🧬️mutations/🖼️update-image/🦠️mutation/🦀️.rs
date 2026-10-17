//! 🖼️ Atomic image asset and size edits with preserved layer identity.
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-image")]
pub struct UpdateImage {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub image_key: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub width: f64,
    pub height: f64,
}
pub fn update_image<Op: From<UpdateImage>>(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, image_key: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, width: f64, height: f64) -> Op {
    Op::from(UpdateImage { layer_id, image_key, width, height })
}
impl<Op> protocol::MutationKind<DrawingSnapshot, Op> for UpdateImage where Op: protocol::Mutation<DrawingSnapshot, Diff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<UpdateImage> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "image", kind: "update-image", record: "UpdatedImage" };
    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<semio_s_artifact_draw_drawing_diff::DrawingDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Edit image", "Bild bearbeiten") }
    fn target(&self) -> Vec<String> { vec![self.layer_id.to_string_owner()] }
}
