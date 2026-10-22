//! 🔷️ Atomic authored shape coordinate edits with preserved layer identity.
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "set-shape-coordinate")]
pub struct SetShapeCoordinate {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub field: semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField,
    pub index: Option<usize>,
    pub value: f64,
}
pub fn set_shape_coordinate<Op: From<SetShapeCoordinate>>(layer_id:semio_framework_value::paged::PagedUtf8<{usize::MAX}>,field:semio_s_artifact_draw_drawing_model::shape_geometry::ShapeCoordinateField,index:Option<usize>,value:f64) -> Op {
    Op::from(SetShapeCoordinate {layer_id,field,index,value})
}
impl<Op> protocol::MutationKind<DrawingSnapshot, Op> for SetShapeCoordinate where Op: protocol::Mutation<DrawingSnapshot, Diff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<SetShapeCoordinate> {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "shape", kind: "set-shape-coordinate", record: "SetShapeCoordinate" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Edit shape geometry", "Formgeometrie bearbeiten") }
    fn target(&self) -> Vec<String> { vec![self.layer_id.to_string_owner()] }
}

impl<Op> protocol::ColdMutationKind<DrawingSnapshot, Op> for SetShapeCoordinate where Op: protocol::ColdMutation<DrawingSnapshot, ReferenceDiff = semio_s_artifact_draw_drawing_diff::DrawingDiff> + From<SetShapeCoordinate> {
    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<semio_s_artifact_draw_drawing_diff::DrawingDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
}
