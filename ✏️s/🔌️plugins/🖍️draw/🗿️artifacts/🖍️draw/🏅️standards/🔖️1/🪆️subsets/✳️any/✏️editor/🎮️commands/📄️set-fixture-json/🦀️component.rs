//! 📄️ 📄️ Draw play app commands command — `set-fixture-json`.

use crate::artifacts::draw::op::DrawMutation;
use crate::artifacts::draw::{DrawSnapshot, DRAW_DOCUMENT_SCHEMA};
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "fixture-json")]
pub struct SetFixtureJson {
    pub json: String,
}

/// 🌡 Parsed as JSON (falling back to a no-op when it isn't valid or doesn't carry the draw schema)
/// — mirrors every other plugin's fixture-injection command.
pub fn handle(
    payload: &SetFixtureJson,
    _doc: &ArtifactView<'_, DrawSnapshot>,
    _cfg: &ConfigView<'_, DrawConfig>,
    _session: &mut crate::editor::draw::commands::canvas_pointer_down::DrawSession,
) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    if payload.json.contains(DRAW_DOCUMENT_SCHEMA) {
        if let Ok(snapshot) = serde_json::from_str::<DrawSnapshot>(&payload.json) {
            return Ok(Emit { effects: vec![crate::editor::draw::draw_reset_document_effect(&snapshot)], ..Default::default() });
        }
    }
    Ok(Emit::default())
}
