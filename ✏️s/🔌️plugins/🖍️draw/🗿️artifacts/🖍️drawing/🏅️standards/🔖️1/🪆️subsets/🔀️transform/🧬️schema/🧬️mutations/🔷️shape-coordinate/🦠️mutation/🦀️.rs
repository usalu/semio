//! 🔷️ Atomic authored shape coordinate edits with preserved layer identity.
use crate::{DrawingSnapshot, DrawingMutation};
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "set-shape-coordinate")]
pub struct SetShapeCoordinate {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    pub field: crate::schema::shape_geometry::ShapeCoordinateField,
    pub index: Option<usize>,
    pub value: f64,
}
pub fn set_shape_coordinate(layer_id:semio_framework_value::paged::PagedUtf8<{usize::MAX}>,field:crate::schema::shape_geometry::ShapeCoordinateField,index:Option<usize>,value:f64)->DrawingMutation {
    DrawingMutation::SetShapeCoordinate(SetShapeCoordinate {layer_id,field,index,value})
}
impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for SetShapeCoordinate {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "shape", kind: "set-shape-coordinate", record: "SetShapeCoordinate" };
    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Edit shape geometry", "Formgeometrie bearbeiten") }
    fn target(&self) -> Vec<String> { vec![self.layer_id.to_string_owner()] }
}
