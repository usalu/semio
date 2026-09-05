//! 📄️ 📄️ Drawing play app commands command — `set-fixture-json`.

use crate::artifacts::drawing::op::DrawingMutation;
use crate::artifacts::drawing::{DrawingSnapshot, DRAWING_DOCUMENT_SCHEMA};
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "fixture-json")]
pub struct SetFixtureJson {
    pub json: String,
}

/// 🌡 Parsed as JSON (falling back to a no-op when it isn't valid or doesn't carry the drawing schema)
/// — mirrors every other plugin's fixture-injection command.
pub fn handle(
    payload: &SetFixtureJson,
    _doc: &ArtifactView<'_, DrawingSnapshot>,
    _cfg: &ConfigView<'_, DrawingConfig>,
    _session: &mut crate::editor::drawing::commands::canvas_pointer_down::DrawingSession,
) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    if payload.json.contains(DRAWING_DOCUMENT_SCHEMA) {
        if let Ok(snapshot) = dsl::json::from_json_str::<DrawingSnapshot>(&payload.json) {
            return Ok(Emit { effects: vec![crate::editor::drawing::drawing_reset_document_effect(&snapshot)], ..Default::default() });
        }
    }
    Ok(Emit::default())
}
