//! 🗃️ Note play app commands — whole-document loads (the "semio" example, and raw fixture JSON/DSL
//! import).

use crate::apps::note::config::{NoteConfig, NoteConfigOperation};
use crate::artifacts::note::engine::{empty_note_document, semio_example_document};
use crate::artifacts::note::op::NoteOperation;
use crate::artifacts::note::{NoteDocument, NOTE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, _doc: &DocumentView<'_, NoteDocument>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteOperation, NoteConfigOperation>, Fault> {
        let next_document = if payload.example_id == "semio" { semio_example_document() } else { empty_note_document() };
        Ok(Emit { document_operations: vec![NoteOperation::SetDocument { document: next_document }], config_operations: vec![NoteConfigOperation::SetSelection { block_ids: Vec::new() }], ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

//#region 🔖️SetFixtureJson
pub mod set_fixture_json {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "set-fixture-json")]
    pub struct SetFixtureJson {
        pub json: String,
    }

    pub fn handle(payload: &SetFixtureJson, _doc: &DocumentView<'_, NoteDocument>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteOperation, NoteConfigOperation>, Fault> {
        let next_document = if let Ok(document) = crate::artifacts::note::dsl::parse_dsl(&payload.json) {
            document
        } else {
            let Ok(parsed) = serde_json::from_str::<Value>(&payload.json) else {
                return Ok(Emit::default());
            };
            if parsed.get("schema").and_then(|value| value.as_str()) != Some(NOTE_DOCUMENT_SCHEMA) {
                return Ok(Emit::default());
            }
            let Ok(document) = serde_json::from_value::<NoteDocument>(parsed) else {
                return Ok(Emit::default());
            };
            document
        };
        Ok(Emit { document_operations: vec![NoteOperation::SetDocument { document: next_document }], config_operations: vec![NoteConfigOperation::SetSelection { block_ids: Vec::new() }], ..Default::default() })
    }
}
//#endregion 🔖️SetFixtureJson

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::testkit::{dispatch, note_app};
    use crate::apps::note::NoteCommand;

    #[test]
    fn set_fixture_json_replaces_document() {
        let mut app = note_app();
        let result = dispatch(&mut app, NoteCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: crate::artifacts::note::engine::semio_example_json() }));
        assert_eq!(result.operations.len(), 1);
        assert_eq!(app.projection().expect("projection").blocks.len(), 3);
    }

    #[test]
    fn set_active_example_loads_semio_blocks() {
        let mut app = note_app();
        dispatch(&mut app, NoteCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "semio".into() }));
        assert_eq!(app.projection().expect("projection").blocks.len(), 3);

        dispatch(&mut app, NoteCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: String::new() }));
        assert!(app.projection().expect("projection").blocks.is_empty());
    }
}
//#endregion 🧪️Tests
