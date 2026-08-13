//! 🗃️ 🗃️ Note play app commands command — `set-active-example`.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::schema::{empty_note_snapshot, semio_example_snapshot};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let next_document = if payload.example_id == "semio" { semio_example_snapshot() } else { empty_note_snapshot() };
    Ok(Emit { effects: vec![crate::apps::note::reset_document_effect(&next_document)], config_mutations: vec![NoteConfigMutation::SetSelection { block_ids: Vec::new() }], ..Default::default() })
}

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
        let doc = ArtifactView::new(&snapshot, &history);
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
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = NoteConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };

        let emit = handle(&SetActiveExample { example_id: "semio".into() }, &doc, &cfg).expect("handle");
        let HostEffect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <NoteSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert_eq!(loaded.blocks.len(), 1);

        let emit = handle(&SetActiveExample { example_id: String::new() }, &doc, &cfg).expect("handle");
        let HostEffect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <NoteSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(loaded.blocks.is_empty());
    }
}
//#endregion 🧪️Tests
