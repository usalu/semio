//! 🖊️ Note play app command — batched ink-canvas events (add/update/remove block, put asset, set
//! camera). The sole content-mutating entry point the ink-canvas host surface calls.

use crate::apps::note::config::{NoteConfig, NoteConfigMutation};
use crate::artifacts::note::engine::{insert_block, remove_block_from_tree, update_block_in_tree};
use crate::artifacts::note::op::NoteMutation;
use crate::artifacts::note::{NoteBlockNode, NoteCamera, NoteSnapshot, NoteImageAsset};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::Deserialize;

//#region 🔖️CanvasEvents
/// 🖱️ Batched canvas-event wire shape the `ink-canvas-host` surface emits (`addBlock`/`updateBlock`/
/// `removeBlock`/`putAsset`/`setCamera`); content events diff into `NoteMutation`s via
/// `note_ops_from_canvas_events`, `setCamera` diffs into a `NoteConfigMutation::SetCamera` instead
/// (session-only view state, never a document field).
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "mutation")]
enum NoteCanvasEvent {
    #[serde(rename = "addBlock", rename_all = "camelCase")]
    AddBlock {
        block: NoteBlockNode,
        #[serde(default)]
        parent_id: Option<String>,
        #[serde(default)]
        index: Option<usize>,
    },
    #[serde(rename = "updateBlock", rename_all = "camelCase")]
    UpdateBlock { block_id: String, block: NoteBlockNode },
    #[serde(rename = "removeBlock", rename_all = "camelCase")]
    RemoveBlock { block_id: String },
    #[serde(rename = "putAsset", rename_all = "camelCase")]
    PutAsset { key: String, asset: NoteImageAsset },
    #[serde(rename = "setCamera", rename_all = "camelCase")]
    SetCamera { camera: NoteCamera },
}

fn apply_note_canvas_event(document: &mut NoteSnapshot, event: &NoteCanvasEvent) {
    match event {
        NoteCanvasEvent::AddBlock { block, parent_id, index } => {
            insert_block(&mut document.blocks, parent_id.as_deref(), index.unwrap_or(usize::MAX), block.clone());
        }
        NoteCanvasEvent::UpdateBlock { block_id, block } => {
            update_block_in_tree(&mut document.blocks, block_id, block.clone());
        }
        NoteCanvasEvent::RemoveBlock { block_id } => {
            remove_block_from_tree(&mut document.blocks, block_id);
        }
        NoteCanvasEvent::PutAsset { key, asset } => {
            document.assets.insert(key.clone(), asset.clone());
        }
        // 📷️ Camera never touches the document — `inkApplyEvents` pulls it into runtime state before
        // this function ever sees the batch (see the `NoteCanvasEvent::SetCamera` filter there).
        NoteCanvasEvent::SetCamera { .. } => {}
    }
}

/// 🔀️ Applies a batch of canvas events to a cloned document and returns the minimal `NoteMutation`s
/// describing what changed (block-tree snapshot and per-asset puts) — the empty vec means no content
/// changed (e.g. a gesture that ended where it began).
fn note_ops_from_canvas_events(document: &NoteSnapshot, events: &[NoteCanvasEvent]) -> Vec<NoteMutation> {
    let mut next = document.clone();
    for event in events {
        apply_note_canvas_event(&mut next, event);
    }
    let mut operations = Vec::new();
    if next.blocks != document.blocks {
        operations.push(NoteMutation::SetBlocks { blocks: next.blocks.clone() });
    }
    for (key, asset) in &next.assets {
        if document.assets.get(key) != Some(asset) {
            operations.push(NoteMutation::PutAsset { key: key.clone(), asset: asset.clone() });
        }
    }
    operations
}
//#endregion 🔖️CanvasEvents

//#region 🔖️InkApplyEvents
pub mod ink_apply_events {
    use super::*;

    #[derive(Clone, Debug, PartialEq, serde::Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "ink-apply-events")]
    pub struct InkApplyEvents {
        pub events_json: String,
        pub phase: String,
        pub select_ids: Option<Vec<String>>,
    }

    pub fn handle(payload: &InkApplyEvents, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
        let document = doc.snapshot;
        let events: Vec<NoteCanvasEvent> = serde_json::from_str(&payload.events_json).unwrap_or_default();
        let mut config_mutations = Vec::new();
        if let Some(ids) = &payload.select_ids {
            config_mutations.push(NoteConfigMutation::SetSelection { block_ids: ids.clone() });
        }
        // 📷️ Camera rides in the same batch as content edits but never becomes a document operation —
        // diffs into a config operation instead.
        for event in &events {
            if let NoteCanvasEvent::SetCamera { camera } = event {
                config_mutations.push(NoteConfigMutation::SetCamera { camera: camera.clone() });
            }
        }
        let operations = note_ops_from_canvas_events(document, &events);
        if operations.is_empty() && config_mutations.is_empty() {
            return Ok(Emit::default());
        }
        // The whole drag (begin → live* → commit) coalesces into ONE undoable edit; a lone `atomic`
        // event batch is its own edit. Selection/camera-only batches (no content change) never need
        // coalescing.
        let coalesce_key = if operations.is_empty() {
            None
        } else {
            match payload.phase.as_str() {
                "begin" | "live" | "commit" => Some("note-gesture".into()),
                _ => None,
            }
        };
        Ok(Emit { artifact_mutations: operations, config_mutations, coalesce_key, ..Default::default() })
    }
}
//#endregion 🔖️InkApplyEvents

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::note::testkit::{dispatch, note_app};
    use crate::apps::note::NoteCommand;
    use crate::artifacts::note::engine::{block_id, create_block_by_kind};
    use semio_framework_plugin::PluginApp;
    use serde_json::json;

    #[test]
    fn gesture_begin_live_commit_produces_single_undo_step() {
        let mut app = note_app();
        let block = create_block_by_kind("text", 10.0, 10.0);
        let new_id = block_id(&block).to_string();

        let begin_events = json!([
            { "mutation": "addBlock", "block": block, "parentId": null, "index": null }
        ])
        .to_string();
        dispatch(&mut app, NoteCommand::InkApplyEvents(ink_apply_events::InkApplyEvents { events_json: begin_events, phase: "begin".into(), select_ids: Some(vec![new_id.clone()]) }));
        assert_eq!(app.snapshot().expect("snapshot").blocks.len(), 1);

        for x in [20.0, 30.0, 40.0] {
            let mut moved = block.clone();
            if let NoteBlockNode::Text { x: block_x, .. } = &mut moved {
                *block_x = x;
            }
            let live_events = json!([
                { "mutation": "updateBlock", "blockId": new_id, "block": moved }
            ])
            .to_string();
            dispatch(&mut app, NoteCommand::InkApplyEvents(ink_apply_events::InkApplyEvents { events_json: live_events, phase: "live".into(), select_ids: None }));
        }
        assert_eq!(app.snapshot().expect("snapshot").blocks.len(), 1);

        // Commit with no further change emits no operation — the gesture is already recorded.
        let commit = dispatch(&mut app, NoteCommand::InkApplyEvents(ink_apply_events::InkApplyEvents { events_json: "[]".into(), phase: "commit".into(), select_ids: None }));
        assert!(commit.mutations.is_empty(), "a no-operation commit must not create an edit");
        assert_eq!(app.snapshot().expect("snapshot").blocks.len(), 1);

        // The whole begin+live gesture coalesced into ONE undoable edit.
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert!(app.snapshot().expect("snapshot").blocks.is_empty(), "a single undo should erase the whole gesture");
    }

    #[test]
    fn gesture_with_no_changes_creates_no_edit() {
        let mut app = note_app();
        dispatch(&mut app, NoteCommand::InkApplyEvents(ink_apply_events::InkApplyEvents { events_json: "[]".into(), phase: "begin".into(), select_ids: None }));
        dispatch(&mut app, NoteCommand::InkApplyEvents(ink_apply_events::InkApplyEvents { events_json: "[]".into(), phase: "commit".into(), select_ids: None }));
        let undo = app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert!(undo.events.is_empty(), "no gesture edit should exist to undo");
    }
}
//#endregion 🧪️Tests
