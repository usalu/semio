//! 🗃️ 🗃️ Note play app commands command — `load-document-json`.

use crate::op::NoteMutation;
use crate::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "load-document-json")]
pub struct LoadDocumentJson {
    pub json: String,
}

pub fn handle(payload: &LoadDocumentJson, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, semio_framework_plugin::NoConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, semio_framework_plugin::NoConfigMutation>, Fault> {
    let next_document = if let Ok(document) = crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(&payload.json) {
        document
    } else {
        let Ok(parsed) = serde_json::from_str::<Value>(&payload.json) else {
            return Ok(Emit::default());
        };
        if parsed.get("schema").and_then(|value| value.as_str()) != Some(NOTE_DOCUMENT_SCHEMA) {
            return Ok(Emit::default());
        }
        let Ok(document) = semio_framework_pack_json::from_json_str::<NoteSnapshot>(&payload.json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
            return Ok(Emit::default());
        };
        document
    };
    Ok(Emit { effects: vec![crate::editor::note::reset_document_effect(&next_document)], ..Default::default() })
}
