//! 🗃️ 🗃️ Note play app commands command — `set-active-example`.

use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::schema::{empty_note_snapshot, semio_example_snapshot};
use crate::artifacts::note::{NoteSnapshot, NOTE_DOCUMENT_SCHEMA};
use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde_json::Value;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "set-active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

pub async fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, _ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let next_document = if payload.example_id == "semio" { semio_example_snapshot() } else { empty_note_snapshot() };
    Ok(Emit { effects: vec![crate::editor::note::reset_document_effect(&next_document)], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::note::commands::set_fixture_json;
    use semio_framework::kernel::Effect;

    /// 🧬️ Driven directly through `handle` (not `dispatch`, which routes through `VcsArtifactApp` and
    /// never applies `effects` to its own store — that's the real host's job): asserts on the `Emit`
    /// itself, mirroring `fem2d`'s `set_active_example` test of the same `Effect::LoadDocument`
    /// reroute (whole-document replace is banned from the `Mutation` enum outright).
    async fn empty_view() -> (NoteSnapshot, semio_framework_plugin::HistoryView) {
        (empty_note_snapshot(), semio_framework_plugin::HistoryView::empty())
    }

    #[semio_framework_async_macros::async_test]
    async fn set_fixture_json_replaces_document() {
        let (snapshot, history) = empty_view();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = NoteConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let mut ctx = crate::editor::note::NoteDispatchCtx { selected_block_ids: Vec::new(), id_owner: crate::artifacts::note::schema::NoteIdOwner::new("active-example-test", 0) };
        let emit = set_fixture_json::handle(&set_fixture_json::SetFixtureJson { json: crate::artifacts::note::schema::semio_example_json() }, &doc, &cfg, &mut ctx).expect("handle");
        assert!(emit.artifact_mutations.is_empty(), "whole-document load must not go through the Mutation enum");
        let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setFixtureJson must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <NoteSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("decode loaded document pack");
        assert_eq!(loaded.blocks.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_example_loads_semio_blocks() {
        let (snapshot, history) = empty_view();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = NoteConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let mut ctx = crate::editor::note::NoteDispatchCtx { selected_block_ids: Vec::new(), id_owner: crate::artifacts::note::schema::NoteIdOwner::new("active-example-test", 0) };

        let emit = handle(&SetActiveExample { example_id: "semio".into() }, &doc, &cfg, &mut ctx).expect("handle");
        let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <NoteSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert_eq!(loaded.blocks.len(), 1);

        let emit = handle(&SetActiveExample { example_id: String::new() }, &doc, &cfg, &mut ctx).expect("handle");
        let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("setActiveExample must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <NoteSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(loaded.blocks.is_empty());
    }
}
//#endregion 🧪️Tests
