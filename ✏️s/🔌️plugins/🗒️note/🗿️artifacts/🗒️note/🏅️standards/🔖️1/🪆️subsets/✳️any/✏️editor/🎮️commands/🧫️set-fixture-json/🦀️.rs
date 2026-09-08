//! 🗃️ 🗃️ Note play app commands command — `set-fixture-json`.

use crate::op::NoteMutation;
use crate::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde_json::Value;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-fixture-json")]
pub struct SetFixtureJson {
    pub json: String,
}

pub fn handle(payload: &SetFixtureJson, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let next_document = if let Ok(document) = crate::dsl::parse_dsl(&payload.json) {
        document
    } else {
        let Ok(parsed) = serde_json::from_str::<Value>(&payload.json) else {
            return Ok(Emit::default());
        };
        if parsed.get("schema").and_then(|value| value.as_str()) != Some(NOTE_DOCUMENT_SCHEMA) {
            return Ok(Emit::default());
        }
        let Ok(document) = dsl::os_pack::from_json_str::<NoteSnapshot>(&payload.json) else {
            return Ok(Emit::default());
        };
        document
    };
    Ok(Emit { effects: vec![crate::editor::note::reset_document_effect(&next_document)], ..Default::default() })
}
