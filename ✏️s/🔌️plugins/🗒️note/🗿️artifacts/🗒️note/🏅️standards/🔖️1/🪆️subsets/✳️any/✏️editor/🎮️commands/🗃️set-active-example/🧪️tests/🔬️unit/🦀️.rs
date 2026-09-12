use super::*;
use crate::editor::note::commands::set_fixture_json;
use semio_framework::kernel::Effect;

/// 🧬️ Driven directly through `handle` (not `dispatch`, which routes through `VcsArtifactApp` and
/// never applies `effects` to its own store — that's the real host's job): asserts on the `Emit`
/// itself, mirroring `fem2d`'s `set_active_example` test of the same `Effect::LoadDocument`
/// reroute (whole-document replace is banned from the `Mutation` enum outright).
fn empty_view() -> (NoteSnapshot, semio_framework_plugin::HistoryView) {
    (empty_note_snapshot(), semio_framework_plugin::HistoryView::empty())
}

#[semio_framework_async_macros::async_test]
async fn set_fixture_json_replaces_document() {
    let (snapshot, history) = empty_view();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg_snapshot = semio_framework_plugin::NoConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let mut ctx = crate::editor::note::NoteDispatchCtx { selected_block_ids: Vec::new(), id_owner: crate::schema::NoteIdOwner::new("active-example-test", 0), view_state: None, window_transient: Default::default(), window_transient_owner: None };
    let emit = set_fixture_json::handle(&set_fixture_json::SetFixtureJson { json: crate::schema::semio_example_json() }, &doc, &cfg, &mut ctx).expect("handle");
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
    let cfg_snapshot = semio_framework_plugin::NoConfig::default();
    let cfg = ConfigView { snapshot: &cfg_snapshot, window: None };
    let mut ctx = crate::editor::note::NoteDispatchCtx { selected_block_ids: Vec::new(), id_owner: crate::schema::NoteIdOwner::new("active-example-test", 0), view_state: None, window_transient: Default::default(), window_transient_owner: None };

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
