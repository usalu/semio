//! 📄️ 📄️ Draw play app commands command — `set-active-example`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::schema::{default_draw_document, semio_draw_example_document};
use crate::artifacts::draw::DrawSnapshot;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use crate::editor::draw::DRAW_PLAY_EXAMPLE_DEFAULT_ID;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

pub fn handle(
    payload: &SetActiveExample,
    _doc: &ArtifactView<'_, DrawSnapshot>,
    _cfg: &ConfigView<'_, DrawConfig>,
    _session: &mut crate::editor::draw::commands::canvas_pointer_down::DrawSession,
) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let next = if payload.example_id.is_empty() {
        Some(default_draw_document("empty", None))
    } else if payload.example_id == DRAW_PLAY_EXAMPLE_DEFAULT_ID {
        Some(semio_draw_example_document())
    } else {
        None
    };
    match next {
        Some(snapshot) => Ok(Emit { effects: vec![crate::editor::draw::draw_reset_document_effect(&snapshot)], ..Default::default() }),
        None => Ok(Emit::default()),
    }
}
