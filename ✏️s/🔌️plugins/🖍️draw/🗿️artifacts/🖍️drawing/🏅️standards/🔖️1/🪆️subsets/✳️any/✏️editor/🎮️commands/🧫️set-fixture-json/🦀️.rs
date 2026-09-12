//! 📄️ 📄️ Drawing play app commands command — `set-fixture-json`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use crate::op::DrawingMutation;
use crate::{DrawingSnapshot, DRAWING_DOCUMENT_SCHEMA};
use dsl::{FromValue, ToValue};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

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
    _cfg: &ConfigView<'_, NoConfig>,
    _session: &mut crate::editor::drawing::commands::canvas_pointer_down::DrawingSession,
) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    if payload.json.contains(DRAWING_DOCUMENT_SCHEMA) {
        if let Ok(snapshot) = dsl::json::from_json_str::<DrawingSnapshot>(&payload.json) {
            return Ok(Emit { effects: vec![crate::editor::drawing::drawing_reset_document_effect(&snapshot)], ..Default::default() });
        }
    }
    Ok(Emit::default())
}
