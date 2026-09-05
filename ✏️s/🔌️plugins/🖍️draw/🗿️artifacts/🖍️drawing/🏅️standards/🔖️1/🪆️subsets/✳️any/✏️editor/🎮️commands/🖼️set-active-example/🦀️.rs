//! 📄️ 📄️ Drawing play app commands command — `set-active-example`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::schema::{default_drawing_document, semio_drawing_example_document};
use crate::artifacts::drawing::DrawingSnapshot;
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use crate::editor::drawing::DRAWING_PLAY_EXAMPLE_DEFAULT_ID;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

pub fn handle(
    payload: &SetActiveExample,
    _doc: &ArtifactView<'_, DrawingSnapshot>,
    _cfg: &ConfigView<'_, DrawingConfig>,
    _session: &mut crate::editor::drawing::commands::canvas_pointer_down::DrawingSession,
) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let next = if payload.example_id.is_empty() {
        Some(default_drawing_document("empty", None))
    } else if payload.example_id == DRAWING_PLAY_EXAMPLE_DEFAULT_ID {
        Some(semio_drawing_example_document())
    } else {
        None
    };
    match next {
        Some(snapshot) => Ok(Emit { effects: vec![crate::editor::drawing::drawing_reset_document_effect(&snapshot)], ..Default::default() }),
        None => Ok(Emit::default()),
    }
}
