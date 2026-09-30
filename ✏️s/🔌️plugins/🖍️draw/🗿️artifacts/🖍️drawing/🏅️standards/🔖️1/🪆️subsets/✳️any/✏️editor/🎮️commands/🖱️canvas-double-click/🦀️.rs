//! 🖱️ 🖱️ Drawing play app commands command — `canvas-double-click`.

use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-double-click")]
pub struct CanvasDoubleClick {}

/// ✅️ Commits the open pen or polygon draft as one canvas tool transaction.
pub fn handle(_payload: &CanvasDoubleClick, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let base = session.tool_base(doc);
    session.finish_draft(base)
}
