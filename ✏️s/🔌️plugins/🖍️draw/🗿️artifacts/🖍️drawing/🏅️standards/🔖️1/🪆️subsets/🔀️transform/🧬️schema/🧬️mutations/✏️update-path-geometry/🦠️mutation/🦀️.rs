//! ✏️ Atomic semantic replacement of one path's geometry, preserving its identity and appearance.
use crate::{DrawingSnapshot, PathSegment};
use crate::mutations::DrawingMutation;
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "update-path-geometry")]
pub struct UpdatePathGeometry {
    pub layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>,
    #[dsl(statements, block)]
    pub segments: semio_framework_value::list::PagedList<PathSegment, {usize::MAX}>,
}
pub fn update_path_geometry(layer_id: semio_framework_value::paged::PagedUtf8<{usize::MAX}>, segments: semio_framework_value::list::PagedList<PathSegment, {usize::MAX}>) -> DrawingMutation {
    DrawingMutation::UpdatePathGeometry(UpdatePathGeometry { layer_id, segments })
}
impl protocol::MutationKind<DrawingSnapshot, DrawingMutation> for UpdatePathGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "path", kind: "update-path-geometry", record: "UpdatedPathGeometry" };
    fn diff(&self, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> { super::diff::diff(self, base) }
    fn inverse(&self, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok({ super::inverse::inverse(self, base)? 
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Edit path", "Pfad bearbeiten") }
    fn target(&self) -> Vec<String> { vec![self.layer_id.to_string_owner()] }
}
