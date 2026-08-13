//! 🗃️ 🗃️ Note play app commands command — `set-fixture-json`.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::schema::{empty_note_snapshot, semio_example_snapshot};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-fixture-json")]
pub struct SetFixtureJson {
    pub json: String,
}

pub fn handle(payload: &SetFixtureJson, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let next_document = if let Ok(document) = crate::artifacts::note::dsl::parse_dsl(&payload.json) {
        document
    } else {
        let Ok(parsed) = serde_json::from_str::<Value>(&payload.json) else {
            return Ok(Emit::default());
        };
        if parsed.get("schema").and_then(|value| value.as_str()) != Some(NOTE_DOCUMENT_SCHEMA) {
            return Ok(Emit::default());
        }
        let Ok(document) = serde_json::from_value::<NoteSnapshot>(parsed) else {
            return Ok(Emit::default());
        };
        document
    };
    Ok(Emit { effects: vec![crate::apps::note::reset_document_effect(&next_document)], config_mutations: vec![NoteConfigMutation::SetSelection { block_ids: Vec::new() }], ..Default::default() })
}
