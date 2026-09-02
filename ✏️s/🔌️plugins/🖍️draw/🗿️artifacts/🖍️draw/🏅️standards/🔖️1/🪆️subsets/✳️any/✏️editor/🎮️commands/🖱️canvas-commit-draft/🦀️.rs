//! 🖱️ 🖱️ Draw play app commands command — `canvas-commit-draft`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use crate::editor::draw::commands::canvas_pointer_down::{draw_gesture, DrawSession};
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "canvas-commit-draft")]
pub struct CanvasCommitDraft {}

pub fn handle(_payload: &CanvasCommitDraft, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let emit = session.step_gesture(draw_gesture::Event::CommitDraft, document, config);
    Ok(emit)
}
