//! 🖱️ 🖱️ Draw play app commands command — `canvas-double-click`.

use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use crate::editor::draw::commands::canvas_pointer_down::{draw_gesture, DrawSession};
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "canvas-double-click")]
pub struct CanvasDoubleClick {}

pub fn handle(_payload: &CanvasDoubleClick, doc: &ArtifactView<'_, DrawSnapshot>, cfg: &ConfigView<'_, DrawConfig>, session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let emit = session.step_gesture(draw_gesture::Event::CommitDraft, document, config);
    Ok(emit)
}
