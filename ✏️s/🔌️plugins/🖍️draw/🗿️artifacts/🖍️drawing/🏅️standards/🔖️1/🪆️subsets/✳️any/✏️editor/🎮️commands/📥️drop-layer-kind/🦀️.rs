//! 🗂️ 🗂️ Drawing play app commands command — `drop-layer-kind`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "drop-layer-kind")]
pub struct DropLayerKind {
    pub kind: String,
    pub target_row_id: String,
    pub drop_position: String,
}

pub fn handle(payload: &DropLayerKind, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let layer = super::add_layer::build_layer(document, &payload.kind, doc.operation_optional())?;
    let (parent_id, index) = super::move_layer::resolve_reorder_target(document, &payload.target_row_id, &payload.drop_position)?;
    Ok(Emit::commit(vec![crate::mutations::create_layer(parent_id, Some(index), layer)], "Add layer"))
}
