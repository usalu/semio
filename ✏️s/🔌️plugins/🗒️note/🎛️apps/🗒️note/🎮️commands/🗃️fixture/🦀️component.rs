//! 🗃️ Note play app commands — whole-document loads (the "semio" example, and raw fixture JSON/DSL
//! import).

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::schema::{empty_note_snapshot, semio_example_snapshot};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
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

    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        let next_document = if payload.example_id == "semio" { semio_example_snapshot() } else { empty_note_snapshot() };
        Ok(Emit { effects: vec![crate::apps::note::reset_document_effect(&next_document)], config_mutations: vec![NoteConfigMutation::SetSelection { block_ids: Vec::new() }], ..Default::default() })
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
}
//#endregion 🔖️SetFixtureJson

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework::kernel::HostEffect;

    /// 🧬️ Driven directly through `handle` (not `dispatch`, which routes through `VcsArtifactApp` and
    /// never applies `effects` to its own store — that's the real host's job): asserts on the `Emit`
    /// itself, mirroring `fem2d`'s `set_active_example` test of the same `HostEffect::LoadDocument`
    /// reroute (whole-document replace is banned from the `Mutation` enum outright).
    fn empty_view() -> (NoteSnapshot, semio_framework_plugin::HistoryView) {
        (empty_note_snapshot(), semio_framework_plugin::HistoryView::empty())
    }

    #[test]
    fn set_fixture_json_replaces_document() {
        let (snapshot, history) = empty_view();
        let doc = ArtifactView { snapshot: &snapshot, history: &history };
        let cfg_snapshot = NoteConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let emit = set_fixture_json::handle(&set_fixture_json::SetFixtureJson { json: crate::artifacts::note::schema::semio_example_json() }, &doc, &cfg).expect("handle");
        assert!(emit.artifact_mutations.is_empty(), "whole-document load must not go through the Mutation enum");
        let HostEffect::LoadDocument { pack, .. } = emit.effects.first().expect("setFixtureJson must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <NoteSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert_eq!(loaded.blocks.len(), 1);
    }

    #[test]
    fn set_active_example_loads_semio_blocks() {
        let (snapshot, history) = empty_view();
        let doc = ArtifactView { snapshot: &snapshot, history: &history };
        let cfg_snapshot = NoteConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };

        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: "semio".into() }, &doc, &cfg).expect("handle");
        let HostEffect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <NoteSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert_eq!(loaded.blocks.len(), 1);

        let emit = set_active_example::handle(&set_active_example::SetActiveExample { example_id: String::new() }, &doc, &cfg).expect("handle");
        let HostEffect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <NoteSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(loaded.blocks.is_empty());
    }
}
//#endregion 🧪️Tests
