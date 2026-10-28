//! ✏️ Atomic semantic replacement of one path's geometry, preserving its identity and appearance.
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::PathSegment;

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree,semio_framework_value::NativeReceive)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-path-geometry")]
pub struct UpdatePathGeometry {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[dsl(statements, block)]
    pub segments: semio_framework_value_persistent::list::PagedList<PathSegment, {usize::MAX}>,
}
pub fn update_path_geometry<Op: From<UpdatePathGeometry>>(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, segments: semio_framework_value_persistent::list::PagedList<PathSegment, {usize::MAX}>) -> Op {
    Op::from(UpdatePathGeometry { layer_id, segments })
}
impl<Op> protocol::MutationKind<DrawingSnapshot, Op> for UpdatePathGeometry where Op: protocol::Mutation<DrawingSnapshot, Diff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<UpdatePathGeometry> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "path", kind: "update-path-geometry", record: "UpdatedPathGeometry" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Edit path", "Pfad bearbeiten") }
    fn target(&self) -> Vec<String> { vec![self.layer_id.to_string_owner()] }
}

impl<Op> protocol::ColdMutationKind<DrawingSnapshot, Op> for UpdatePathGeometry where Op: protocol::ColdMutation<DrawingSnapshot, ReferenceDiff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<UpdatePathGeometry> {
    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<semio_s_artifact_draw_drawing_diff::DrawingDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
