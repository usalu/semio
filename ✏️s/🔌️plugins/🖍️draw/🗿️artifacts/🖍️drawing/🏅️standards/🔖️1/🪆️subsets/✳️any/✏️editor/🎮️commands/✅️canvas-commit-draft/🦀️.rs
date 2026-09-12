//! 🖱️ 🖱️ Drawing play app commands command — `canvas-commit-draft`.

use crate::editor::drawing::commands::canvas_pointer_down::{drawing_gesture, DrawingSession};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::DrawingSnapshot;
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-commit-draft")]
pub struct CanvasCommitDraft {}

pub fn handle(_payload: &CanvasCommitDraft, doc: &ArtifactView<'_, DrawingSnapshot>, cfg: &ConfigView<'_, NoConfig>, session: &mut DrawingSession) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let emit = session.step_gesture(drawing_gesture::Event::CommitDraft, document, config);
    Ok(emit)
}
