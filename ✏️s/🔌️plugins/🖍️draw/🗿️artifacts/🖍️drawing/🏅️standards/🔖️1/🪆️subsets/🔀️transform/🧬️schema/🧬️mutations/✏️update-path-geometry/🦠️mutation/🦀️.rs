//! ✏️ Atomic semantic replacement of one path's geometry, preserving its identity and appearance.
use crate::{DrawingSnapshot, PathSegment};
use crate::mutations::DrawingMutation;
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-path-geometry")]
pub struct UpdatePathGeometry {
    pub layer_id: String,
    #[dsl(statements, block)]
    pub segments: Vec<PathSegment>,
}
pub fn update_path_geometry(layer_id: String, segments: Vec<PathSegment>) -> DrawingMutation {
    DrawingMutation::UpdatePathGeometry(UpdatePathGeometry { layer_id, segments })
}
impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for UpdatePathGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "path", kind: "update-path-geometry", record: "UpdatedPathGeometry" };
    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &DrawingSnapshot) -> Vec<DrawingMutation> { super::inverse::inverse(self, base) }
    fn label(&self) -> protocol::LocalizedLabel { protocol::LocalizedLabel::native("Edit path", "Pfad bearbeiten") }
    fn target(&self) -> Vec<String> { vec![self.layer_id.clone()] }
}
