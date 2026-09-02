//! 🖱️ 🖱️ Drawing play app commands command — `canvas-commit-draft`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::DrawingSnapshot;
use crate::editor::drawing::commands::canvas_pointer_down::{drawing_gesture, DrawingSession};
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-commit-draft")]
pub struct CanvasCommitDraft {}

pub fn handle(_payload: &CanvasCommitDraft, doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, DrawingConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let emit = session.step_gesture(drawing_gesture::Event::CommitDraft, document, config);
    Ok(emit)
}
