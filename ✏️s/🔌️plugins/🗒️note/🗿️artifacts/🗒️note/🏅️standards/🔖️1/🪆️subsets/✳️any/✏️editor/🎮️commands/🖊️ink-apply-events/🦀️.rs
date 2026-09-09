//! 🖊️ 🖊️ Note play app command command — `ink-apply-events`.

use crate::editor::note::config::{NoteConfig, NoteConfigMutation};
use crate::op::NoteMutation;
use crate::schema::mutations::{change_block_ink_width, change_block_locked, change_block_visible, create_asset, create_block, delete_block, edit_block_ink_stroke, move_block, rename_block, replace_asset_payload, resize_block};
use crate::schema::{block_bounds, block_id, block_locked, block_name, block_visible, find_block, insert_block, remove_block_from_tree, update_block_in_tree};
use crate::{NoteBlockNode, NoteCamera, NoteImageAsset, NoteSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️CanvasEvents
/// 🖱️ Batched canvas-event wire shape the `ink-canvas-host` surface emits (`addBlock`/`updateBlock`/
/// `removeBlock`/`putAsset`/`setCamera`); content events diff into `NoteMutation`s via
/// `note_ops_from_canvas_events`, while `setCamera` publishes to the exact composite-window config.
#[derive(Clone, Debug, FromValue)]
#[value(tag = "mutation")]
enum NoteCanvasEvent {
    #[value(rename = "addBlock", rename_all = "camelCase")]
    AddBlock {
        block: NoteBlockNode,
        #[value(default)]
        parent_id: Option<String>,
        #[value(default)]
        index: Option<usize>,
    },
    #[value(rename = "updateBlock", rename_all = "camelCase")]
    UpdateBlock { block_id: String, block: NoteBlockNode },
    #[value(rename = "removeBlock", rename_all = "camelCase")]
    RemoveBlock { block_id: String },
    #[value(rename = "putAsset", rename_all = "camelCase")]
    PutAsset { key: String, asset: NoteImageAsset },
    #[value(rename = "setCamera", rename_all = "camelCase")]
    SetCamera { camera: NoteCamera },
}

/// 🎯 Which single/pair of narrow mutations turns `before` into `after` for one block — spatial
/// (`move`/`resize`) and flag (`visible`/`locked`/`name`) fields are compared generically; an `Ink`
/// block's `points`/bbox move together as one `edit-block-ink-stroke` (an authored stroke, never
/// split into a move+resize pair) unless only its `stroke_width` changed.
fn block_update_mutations(id: &str, before: &NoteBlockNode, after: &NoteBlockNode) -> Vec<NoteMutation> {
    let mut ops = Vec::new();
    let (bx, by, bw, bh) = block_bounds(before);
    let (ax, ay, aw, ah) = block_bounds(after);
    if let (NoteBlockNode::Ink { points: before_points, stroke_width: before_width, .. }, NoteBlockNode::Ink { points: after_points, stroke_width: after_width, .. }) = (before, after) {
        if before_points != after_points || (bx, by, bw, bh) != (ax, ay, aw, ah) {
            ops.push(edit_block_ink_stroke(id.to_string(), after_points.clone(), ax, ay, aw, ah));
        } else if before_width != after_width {
            ops.push(change_block_ink_width(id.to_string(), *after_width));
        }
    } else {
        if (bx, by) != (ax, ay) {
            ops.push(move_block(id.to_string(), ax, ay));
        }
        if (bw, bh) != (aw, ah) {
            ops.push(resize_block(id.to_string(), aw, ah));
        }
    }
    if block_visible(before) != block_visible(after) {
        ops.push(change_block_visible(id.to_string(), block_visible(after)));
    }
    if block_locked(before) != block_locked(after) {
        ops.push(change_block_locked(id.to_string(), block_locked(after)));
    }
    if block_name(before) != block_name(after) {
        ops.push(rename_block(id.to_string(), block_name(after).to_string()));
    }
    ops
}

/// 🔀️ Applies a batch of canvas events to a cloned document and returns the minimal `NoteMutation`s
/// describing what changed — one `create-block`/`delete-block` per add/remove event, one collapsed
/// update mutation set per touched id (multiple `updateBlock` events on the same id in one batch
/// collapse to the id's net before→after change, matching the old whole-snapshot diff's coalescing),
/// one `create-asset`/`replace-asset-payload` per changed asset key. The empty vec means no content
/// changed (e.g. a gesture that ended where it began).
fn note_ops_from_canvas_events(document: &NoteSnapshot, events: &[NoteCanvasEvent]) -> Vec<NoteMutation> {
    let mut next = document.clone();
    let mut added: Vec<(String, Option<String>, Option<usize>)> = Vec::new();
    let mut removed: Vec<String> = Vec::new();
    let mut touched: Vec<String> = Vec::new();
    for event in events {
        match event {
            NoteCanvasEvent::AddBlock { block, parent_id, index } => {
                insert_block(&mut next.blocks, parent_id.as_deref(), index.unwrap_or(usize::MAX), block.clone());
                added.push((block_id(block).to_string(), parent_id.clone(), *index));
            }
            NoteCanvasEvent::UpdateBlock { block_id: id, block } => {
                update_block_in_tree(&mut next.blocks, id, block.clone());
                if !touched.contains(id) && !added.iter().any(|(added_id, ..)| added_id == id) {
                    touched.push(id.clone());
                }
            }
            NoteCanvasEvent::RemoveBlock { block_id: id } => {
                remove_block_from_tree(&mut next.blocks, id);
                if find_block(&document.blocks, id).is_some() {
                    removed.push(id.clone());
                }
                added.retain(|(added_id, ..)| added_id != id);
                touched.retain(|touched_id| touched_id != id);
            }
            NoteCanvasEvent::PutAsset { key, asset } => {
                next.assets.insert(key.clone(), asset.clone());
            }
            // 📷️ Camera never touches the document — `inkApplyEvents` pulls it into runtime state before
            // this function ever sees the batch (see the `NoteCanvasEvent::SetCamera` filter there).
            NoteCanvasEvent::SetCamera { .. } => {}
        }
    }
    let mut operations = Vec::new();
    for (id, parent_id, index) in &added {
        if let Some(block) = find_block(&next.blocks, id) {
            operations.push(create_block(block.clone(), parent_id.clone(), *index));
        }
    }
    for id in &removed {
        operations.push(delete_block(id.clone()));
    }
    for id in &touched {
        if let (Some(before), Some(after)) = (find_block(&document.blocks, id), find_block(&next.blocks, id)) {
            if before != after {
                operations.extend(block_update_mutations(id, before, after));
            }
        }
    }
    for (key, asset) in &next.assets {
        if document.assets.get(key) != Some(asset) {
            operations.push(if document.assets.contains_key(key) { replace_asset_payload(key.clone(), asset.clone()) } else { create_asset(key.clone(), asset.clone()) });
        }
    }
    operations
}
//#endregion 🔖️CanvasEvents

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "ink-apply-events")]
pub struct InkApplyEvents {
    pub events_json: String,
    pub phase: String,
    pub select_ids: Option<Vec<String>>,
}

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `payload.select_ids` used to become
// the new selection after a gesture (e.g. a freshly-drawn stroke) — selection is framework-owned
// `InteractionState` now, only ever mutated by the framework's own injected `interactionSelect`
// handling, never by an app command's `Emit`; the field stays on the wire (the ink-canvas host still
// sends it) but is no longer acted on.
pub fn handle(payload: &InkApplyEvents, doc: &ArtifactView<'_, NoteSnapshot>, _cfg: &ConfigView<'_, NoteConfig>, ctx: &mut crate::editor::note::NoteDispatchCtx) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {
    let document = doc.snapshot;
    let events: Vec<NoteCanvasEvent> = dsl::os_pack::from_json_str(&payload.events_json).unwrap_or_default();
    let mut window_config_mutations = Vec::new();
    for event in &events {
        if let NoteCanvasEvent::SetCamera { camera } = event {
            let view = ctx.view_state.as_ref().ok_or_else(|| Fault::from("note-composite-window-context-required"))?;
            window_config_mutations.push(crate::editor::note::window::addressed_config(view, crate::editor::note::window::NoteCompositeWindowConfig { camera: camera.clone() })?);
        }
    }
    let operations = note_ops_from_canvas_events(document, &events);
    if operations.is_empty() && window_config_mutations.is_empty() {
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
    Ok(Emit { artifact_mutations: operations, window_config_mutations, coalesce_key, ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
